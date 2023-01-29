mod common {
    pub mod fixture;
    pub mod datamodel;
}

use tungstenite::Message;
use std::collections::BTreeMap;
use surrealdb_client::connection::surreal_ws_conn::SurrealWsConnection;
use surrealdb_client::driver::surreal_driver::SurrealDriver;
use surrealdb_client::connection::model::rpcresponse::RpcResponse;
use common::fixture::{HOST, PORT, USER_NAME, PASSWORD};
use common::datamodel::{employee::Employee, company::Company};

async fn set_up(ns: &str, db: &str) -> SurrealDriver { 
    let mut surreal_conn = SurrealWsConnection::new(HOST.to_string(), PORT, false);
    _ = surreal_conn.connect().await;
    
    let mut driver = SurrealDriver::new(surreal_conn);

    _ = driver.sign_in(USER_NAME, PASSWORD).await;
    _ = driver.use_ns_db(ns, db).await;

    // let mut args = BTreeMap::new();
    // args.insert("ns".to_string(), ns.to_string());
    // let _ = driver.query("remove namespace $ns;", args).await;

    driver
}

async fn clean_up(driver: &mut SurrealDriver, ns: &str) {
    let mut args = BTreeMap::new();
    args.insert("ns".to_string(), ns.to_string());
    _ = driver.query("remove namespace $ns;", args).await;

    driver.disconnect().await;
}

#[tokio::test]
async fn driver_ping_succeeds() {
    let ns = "test_ping";
    let mut driver = set_up(ns, "test").await;

    let result = driver.ping().await;
    
    assert!(result.is_ok());
    clean_up(&mut driver, ns).await;
}

#[tokio::test]
#[ignore = "needs review about result object"]
async fn driver_info_returns_surreal_info() {
    let ns = "test_info";
    let mut driver = set_up(ns, "test").await;

    let result = driver.info().await;
    // todo: returns "{\"id\":\"54ece72a-f853-4d1d-bb64-22ca2bcda9ab\",\"result\":null}",
    // which seems wrong. Need to fix.
    println!("{:#?}", result);
    clean_up(&mut driver, ns).await;
}

#[tokio::test]
async fn driver_sign_in_succeeds() {
    let ns = "test_signin";
    let mut driver = set_up(ns, "test").await;

    let result = driver.sign_in(USER_NAME, PASSWORD).await;

    assert!(result.is_ok());
    clean_up(&mut driver, ns).await;
}

#[tokio::test]
async fn driver_use_ns_db_succeeds() {
    let ns = "test_use";
    let mut driver = set_up(ns, "test").await;

    let result = driver.use_ns_db("test", "test").await;

    assert!(result.is_ok());
    clean_up(&mut driver, ns).await;
}

#[tokio::test]
async fn driver_query_create_single_employee_succeeds() {
    let ns = "test_query_create_single_emp";
    let mut driver = set_up(ns, "test").await;

    let result = driver.query("
        create Employee \
        set firstName = 'John', lastName = 'Thompson'
    ", BTreeMap::new()).await;
    
    let message: Message = result.unwrap();
    let result_inst: Result<RpcResponse<Employee>, serde_json::Error> = match message {
        Message::Text(txt) => {
            println!("{}", txt.as_str());
            serde_json::from_str(txt.as_str())
        },
        _ => {
            serde_json::from_str("")
        }
    };
    
    let rpc_result = result_inst.unwrap();
    
    assert_eq!(rpc_result.result[0].status, "OK");
    assert_eq!(rpc_result.result.len(), 1 as usize);

    clean_up(&mut driver, ns).await;
}

#[tokio::test]
async fn driver_relation_creation_of_employee_to_company_succeeds() {
    let ns = "test_query_create_co_emp";
    let mut driver = set_up(ns, "test").await;

    // setup
    _ = driver.query("
            create company SET name = 'Super Big Corporation'; \
            create employee set firstName = 'John', lastName = 'Franklin'; \
        ", BTreeMap::new()).await;

    _ = driver.query("insert into company (name) values ('Acme')", BTreeMap::new()).await;

    let mut select_co_args = BTreeMap::new();
    select_co_args.insert("co_name".to_string(), "Acme".to_string());
    let selected_co = driver.query("select * from company where name = $co_name", select_co_args).await;
    let company_responses = Company::from_result_message(&selected_co).unwrap();
    let acme_co = Company::get_first(&company_responses).unwrap();

    let mut update_emp_args: BTreeMap<String, String> = BTreeMap::new();
    update_emp_args.insert("last_name".to_string(), "Franklin".to_string());
    update_emp_args.insert("co".to_string(), String::from(&acme_co.id));
    let updated_emp = driver.query("update employee set company = $co where lastName = $last_name", update_emp_args).await;
    let updated_emp_response = Employee::from_result_message(&updated_emp).unwrap();
    let _ = Employee::get_first(&updated_emp_response).unwrap();
    
    let mut select_emp_args = BTreeMap::new();
    select_emp_args.insert("last_name".to_string(), "Franklin".to_string());
    let selected_emp_result = driver.query("select id, firstName, lastName, company.*.name as company from employee where lastName = $last_name", select_emp_args).await;
    let selected_emp_response = Employee::from_result_message(&selected_emp_result).unwrap();
    let franklin_employee = Employee::get_first(&selected_emp_response).unwrap();    

    assert_eq!(franklin_employee.last_name, "Franklin");
    assert_eq!(franklin_employee.company.as_ref().unwrap(), "Acme");

    clean_up(&mut driver, ns).await;
}