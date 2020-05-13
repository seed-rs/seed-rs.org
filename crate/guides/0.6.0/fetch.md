# HTTP Requests (fetch)

We use the [seed::Request](https://docs.rs/seed/0.6.0/seed/browser/service/fetch/struct.Request.html) struct
to make HTTP requests in the browser, wrapping the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API).
To use this, we need to include `futures = "^0.3.4"` and `serde = { version = "^1.0.85", features = ['derive'] }` in `Cargo.toml`. The [Fetch module](https://docs.rs/seed/0.6.0/seed/browser/service/fetch/index.html)
is standalone: It can be used with any wasm-bindgen program.

## Receiving data

Example, where we update the state on initial load (similar to the 
[server_interaction example](https://github.com/seed-rs/seed/tree/master/examples/server_interaction)
) from a server. It demonstrates a `GET` request, and deserializing JSON data. The `server_interaction`
example contains more sample code.

```rust
use seed::{*, prelude::*};

use futures::Future;
use serde::{Serialize, Deserialize};

struct Model {
    pub branch: Option<Branch>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            branch: None,
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Commit {
    pub sha: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Clone)]
enum Msg {
    DataFetched(seed::fetch::ResponseDataResult<Branch>),
    FetchData,
}

fn fetch_data() -> impl Future<Output = Result<Msg, Msg>> {
    let url = "https://api.github.com/repos/seed-rs/seed/branches/master";
    Request::new(url).fetch_json_data(Msg::DataFetched)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchData => {
            orders.perform_cmd(fetch_data());
            orders.skip();
        }

        Msg::DataFetched(Ok(branch)) => model.branch = Some(branch),

        Msg::DataFetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Fetching repository info failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    if let Some(branch) = &model.branch {
        div![format!(
            "Repo info: Name: {}, SHA: {}",
            branch.name, branch.commit.sha
        )]
    } else {
        div!["Branch not loaded"]
    }
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.send_msg(Msg::FetchData);
    AfterMount::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}

```
After component mount, we trigger an update in the `after_mount` function by sending `Msg::FetchData`, 
which instructs the `update` fn to use `orders.perform_cmd` and an async function we've created
called `fetch_data`. This allows state to be
updated asynchronously, when the request is complete. `skip()` is a convenience method that
sets `Update::ShouldRender` to `Skip`; sending the request doesn't trigger a render.
We pattern-match the response in the `update` function's`DataFetched` arm: If successful, we update the model.
If not, we display an error in the console using the `error!` macro.

We've set up nested structs that have fields matching the names of the JSON fields of
the response, which `Serde` deserializes the response into, through the `fetch_json` method of
 `Request`. Note that even though more data than 
what's contained in our Branch struct is included
in the response, Serde automatically applies only the info matching our struct's fields.

 If we wish to trigger
this update from a normal event instead of on load, we can do something like this:
```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        div![format!(
            "Repo info: Name: {}, SHA: {}",
            model.branch.name, model.branch.commit.sha
        )],
        button![ raw_ev(Ev::Click, move |_| Msg::FetchData), "Update"]
    ]
}
```


## Sending data

Example showing a POST request where we send data to a server and receive the response, 
and a header:
```rust
#[derive(Serialize)]
struct RequestBody {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ResponseBody {
    pub success: bool,
}

#[derive(Clone)]
enum Msg {
    SendMessage,
    MessageSent(seed::fetch::ResponseDataResult<ResponseBody>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SendMessage => {
            orders.skip().perform_cmd(send_message());
        }

        Msg::MessageSent(Ok(response_data)) => {
            log!(format!("Response data: {:#?}", response_data));
            orders.skip();
        }

        Msg::MessageSent(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Sending message failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }
    }
}

fn send_message() -> impl Future<Item = Msg, Error = Msg> {
    let message = RequestBody {
        name: "Mark Watney".into(),
        email: "mark@crypt.kk".into(),
        message: "I wanna be like Iron Man".into(),
    };

    Request::new(CONTACT_URL)
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(Msg::MessageSent)
}

fn view(model: &Model) -> Node<Msg> {
    button![
        simple_ev(Ev::Click, Msg::SendMessage),
        "Send an urgent message (see console log)"
    ]
}

```

Reference the `Request` API docs (linked above) for a full
list of methods available to configure the request, and links to the `MDN` docs describing
them. (eg: `credentials`, `mode`, `integrity`)
