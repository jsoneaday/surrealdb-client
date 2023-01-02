use std::collections::BTreeMap;

#[derive(Debug)]
pub enum RouterMessage {
    SignIn {
        username: String,
        password: String
    },
    UseNsDb {
        ns: String,
        db: String
    },
    Query {
        query_str: String,
        args: BTreeMap<String, String>
    }
}