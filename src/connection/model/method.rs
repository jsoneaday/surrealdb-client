pub enum Method {
    Ping,
    Info,
    Use,
    SignIn,
    Query
}
impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Ping => "ping",
            Method::Info => "info",
            Method::Use => "use",
            Method::SignIn => "signin",
            Method::Query => "query"
        }        
    }
}