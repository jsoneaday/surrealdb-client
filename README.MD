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
