use std::collections::BTreeMap;

use rstest::rstest;
use surrealdb_client::common_tests::datamodel::employee::Employee;
use surrealdb_client::connection::model::rpcresponse::RpcResponse;
use surrealdb_client::connection::model::surrealresult::{SURREALRESULT_STATUS_ERR, SURREALRESULT_STATUS_OK};
use surrealdb_client::router::{
    msg_router_manager::MsgRouterManager,
    message::RouterMessage
};
use surrealdb_client::common_tests::fixtures::globals::{ HOST, PORT, USER_NAME, PASSWORD, NS, DB };


#[tokio::test]
async fn new_msgroutermanager_is_created_without_panic() {
    let _ = MsgRouterManager::build_msg_router_manager(HOST.to_string(), PORT, false).await;        
}

#[rstest]
#[case(RouterMessage::SignIn { username: USER_NAME.to_string(), password: PASSWORD.to_string() })]
#[case(RouterMessage::UseNsDb { ns: NS.to_string(), db: DB.to_string() })]
#[case(RouterMessage::Query { 
    query: "create Employee set firstName = 'John', lastName = 'Thompson'".to_string(), 
    args: BTreeMap::new()
})]
#[tokio::test]
async fn send_message_and_test_valid_response_comes_back_ok(#[case] msg: RouterMessage) {
    let router_manager = MsgRouterManager::build_msg_router_manager(HOST.to_string(), PORT, false).await;
    let result = router_manager.send_msg_to_msg_router_and_wait_receive(msg).await;

    assert!(result.is_err() == false);
    let ok = result.ok();
    assert!(ok != None);
    println!("message: {:?}", ok);
}

#[tokio::test]
async fn send_create_employee_message_and_test_valid_response_comes_back_ok() {
    // note: sometimes failures come back as non-errors with a SurrealResult.status == "ERR"!
    let router_manager = MsgRouterManager::build_msg_router_manager(HOST.to_string(), PORT, false).await;
    let result = router_manager.send_msg_to_msg_router_and_wait_receive(RouterMessage::Query { 
        query: "create Employee set firstName = 'John', lastName = 'Thompson'".to_string(), 
        args: BTreeMap::new()
    }).await;

    assert!(result.is_err() == false);
    let rpc_result_employee = RpcResponse::<Employee>::deserialize(&result.unwrap());    
    assert!(rpc_result_employee.is_err() == false);
    
    let rpc_response_employee = rpc_result_employee.unwrap();
    let first_surreal_result = rpc_response_employee.result.first().clone().unwrap();
    if first_surreal_result.status == SURREALRESULT_STATUS_ERR {
        let None = first_surreal_result.result else {
            panic!("Error: result is not None when status is ERR");
        };
    } else if first_surreal_result.status == SURREALRESULT_STATUS_OK {
        assert!(first_surreal_result.detail == None);
    }
}