# Integration with Rust (backend) servers

If pairing Seed with a Rust backend server, we can simplify passing data between
server and frontend using a layout like that in the 
[server_integration example](https://github.com/David-OConnor/seed/tree/master/examples/server_integration)

A key advantage of this approach is that you can reuse data structures, and code that
operates on them on both client and server. We use `Serde` to elegantly, and mostly transparently,
 handle [de]serialization. For example, we can use use the same struct which represents a 
database model on a server in Seed, without redefining or changing it. This includes
keeping the same methods on both server and client.

The [Engineering Rust Web Applications book](https://erwabook.com/)
 is an excellent resource showing a more detailed layout including a database using
[Diesel](https://diesel.rs), as a step-by-step
 tutorial. You may wish to stop reading this page now, and skip directly to reading
 this book.
 
Highlights:

- We set up three crates, each with its own `Cargo.toml`: One each for server, client, and 
 shared code.
- We place the shared data structures in a barebones third crate called `shared`.
- We set the server and client to use different ports


Folder structure:
```
project folder: 
 └── server: Our Rust server crate, in this case Rocket
 └── client: A normal Seed crate
 └── shared: Contains data structures shared between the server and client
 ```

The top-level project folder contains a `Cargo.toml` that may look like this:
```rust
[workspace]

members = [
    "client",
    "server",
]
```
A makefile, which will may additional scripts from those included
 in the quickstart for running the server, client etc.

Server `Cargo.toml`: A normal one for `Rocket`/`Actix` etc, with a relative-path `shared` dependency
```toml
[package]
name = "server"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[dependencies]
actix = "0.8.3"
actix-web = "1.0.0"
actix-files = "0.1.1"
actix-multipart = "0.1.2"
tokio-timer = "0.2.11"

shared = { path = "../shared" }
```

The client's `cargo.toml` is a standard Seed one. The shared `Cargo.toml` includes
whatever you need for your shared data structures and code; it will usually include
`serde` for serializing and deserializing, and may include database code, since
this crate is a good place for databse models and schema.
```toml
[package]
name = "shared"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[dependencies]
serde = { version = "^1.0.80", features = ['derive'] }
diesel = { version = "^1.4.2", features = ["postgres"] }
```

In `shared/lib.rs`, we set up serializable data structures:
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub val: i8,
    pub text: String,
}
```

In the server and client, we import `shared`, and use these structures normally:

Eg server using `Rocket`:
```rust
use shared::Data;

#[get("/data", format = "application/json")]
fn data_api() -> String {
    let data = Data {
        val: 7,
        text: "Test data".into(),
    };

    serde_json::to_string(&data).unwrap()
}
```

Client, showing how you might use the same struct as part of the model, and
update it from the server:
```rust
use shared::Data;

struct Model {
    pub data: Data,
}

fn get_data() -> impl Future<Item = Msg, Error = Msg> {
    let url = "https://localhost:8001/get_data";

    Request::new(url)
        .method(Method::Get)
        .fetch_json()
        .map(Msg::Replace)
        .map_err(Msg::OnFetchErr)
}

#[derive(Clone)]
enum Msg {
    GetData,
    Replace(Data),
    OnServerResponse(ServerResponse),
    OnFetchErr(JsValue),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Replace(data) => model.data = data,

        Msg::GetData => {
            orders.skip().perform_cmd(get_data());
        }

        Msg::OnServerResponse(result) => {
            log!(format!("Response: {:?}", result));
            orders.skip();
        }

        Msg::OnFetchErr(err) => {
            error!(format!("Fetch error: {:?}", err));
            orders.skip();
        }
    }
}

```