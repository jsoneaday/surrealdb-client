use serde::Deserialize;
use surrealdb_client::connection::model::rpcresponse::RpcResponse;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>
}

impl Employee {
    pub fn from_result_message(msg: &Result<tungstenite::Message, tungstenite::Error>) -> Option<RpcResponse<Employee>> {
        let Ok(message) = msg else {
            return None;
        };
        let employee_result: Result<RpcResponse<Employee>, serde_json::Error> = match message {
            tungstenite::Message::Text(json_txt) => {
                serde_json::from_str(json_txt)
            },
            _ => {
                serde_json::from_str("")
            }
        };
        let Ok(emp_resp) = employee_result else {
            return None;
        };

        Some(emp_resp)
    }

    pub fn get_first<'a>(emp_response: &'a RpcResponse<Employee>) -> Option<&'a Employee> {
        let employee_result = emp_response.result.iter().find(|emp| {
            emp.status == "OK" && emp.result.iter().any(|e| {
                e.last_name == "Franklin"
            })
        });

        employee_result.unwrap().result.first()
    }
}