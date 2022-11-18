# Rust Client Driver for SurrealDB

status: alpha

license: Apache 2.0

**Overview**

This software is broken into two main parts. The SurrealWsConnection and the SurrealDriver. The connection makes the web sockets connection to the remote SurrealDB server.
The driver contains most of the methods for doing queries on SurrealDB.

**Examples** More complete examples are available in the examples folder

Creating a connection object

```
let mut conn = SurrealWsConnection::new(HOST, PORT, false);
let _ = conn.connect().await;
```

Creating a driver object (the main object used to make queries into surrealdb), and then signing in and using a specific namespace and database

```
let mut driver = SurrealDriver::new(conn);
// see file run-docker.sh for starting a surrealdb instance with username and password
let _ = driver.sign_in("superduper", "superpass").await;
let _ = driver.use_ns_db("test", "test").await;
```

Calling any arbitrary query code (this sample shows the create call)

```
let result = driver.query("
    create Person \
    set firstName = 'John', lastName = 'Thompson', age = 18
", BTreeMap::new()).await;
```

**Available Features**

- Signin

  Available now

- Use

  Available now

- Query

  Available now and supports any query

- Select, Insert, Update, Delete

  Coming soon

**Check out the tests for sample code**

**PR's Are Welcome!**
