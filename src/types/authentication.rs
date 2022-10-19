
pub enum Auth<T> {
    Root(RootAuth),
    NS(NamespaceAuth),
    DB(DatabaseAuth),
    Scope(ScopeAuth<T>)
}

#[allow(unused)]
pub struct RootAuth {
    user: String,
    pass: String
}

#[allow(unused)]
pub struct NamespaceAuth {
    ns: String,
    user: String,
    pass: String
}

#[allow(unused)]
pub struct DatabaseAuth {
    ns: String,
    db: String,
    user: String,
    pass: String
}

#[allow(unused)]
pub struct ScopeAuth<T> {    
    ns: String,
    db: String,
    sc: String,
    unknown: T
}