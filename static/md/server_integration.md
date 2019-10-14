# Integration with Rust (backend) servers

# This page is out of date. Standby.

If pairing Seed with a Rust backend server, we can simplify passing data between
server and frontend using a layout like that in the 
[server_integration example](https://github.com/David-OConnor/seed/tree/master/examples/server_integration)
Here, we demonstrate using a single struct for both frontend and server, with `Actix`.
as the server. This is useful for reducing duplication of data structures, and allows
`Serde` to elegantly handle [de]serialization.
For example, we can use use the same struct which represents a 
database item on a server in Seed, without redefining or changing it. This includes
keeping the same methods on both server and client.

Highlights from the example:

- We set up the frontend and backend as independent crates, with the client folder
inside the backend one. Alternatively, we could set them up at the same nest level.
- We place the shared data structures in a barebones third crate called `shared`. We can't access
data on the backend crate due to it being incompatible with the `wasm32-unknown-unknown` target.
We can't do the reverse due to being unable to import `"cdylib"` crates.
- We set the server and client to use different ports
- We are unable to share a workspace between backend and frontend due to incompatible
compile targets.


Folder structure:
```
backend: Our server crate, in this case Rocket
 └── frontend: A normal Seed crate
 └── shared: Contains data structures shared between frontend and backend
 
```

Backend Cargo.toml. A normal `Rocket` one, with a relative-path `shared` dependency, and CORS support.
Notice how we don't use workspaces:
```toml
[package]
name = "backend"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[dependencies]
rocket = "^0.4.0-rc.1"
serde_json = "^1.0.33"
rocket_cors = "^0.4.0"
shared = { path = "shared" }
```

Frontend Cargo.toml. The only difference from a normal Seed crate is the `shared` dependency.
Note that we don't need to import `Serde` directly, in this case.
```toml
[package]
name = "frontend"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
futures = "^0.1.20"
seed = "^0.2.1"
wasm-bindgen = "^0.2.29"
web-sys = "^0.3.6"
shared = { path = "../shared"}
```

Shared Cargo.toml:
```toml
[package]
name = "shared"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[dependencies]
serde = { version = "^1.0.80", features = ['derive'] }
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

In the frontend and backend, we import `shared`, and use these structures normally:

Backend:
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

Frontend, showing how you might use the same data Struct as part of the model, and
update it from the backend:
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