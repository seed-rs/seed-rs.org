# Fetch

_Note_: This chapter only explains Seed `fetch` API, there are no Time Tracker changes.

> How does Seed `fetch` work?

Seed `fetch` is basically a thin wrapper around [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API).

## Example A

Let's look at the code first. This is a part of the [server_integration](https://github.com/seed-rs/seed/blob/480141ce9e520c07e60ddae58244edb40c9f55e9/examples/server_integration/client/src/example_a.rs) example:

```rust
pub enum Msg {
    ...
    SendRequest,
    Fetched(fetch::Result<shared::SendMessageResponseBody>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        ...
        Msg::SendRequest => {
            orders.skip().perform_cmd({
                let message = model.new_message.clone();
                async { Msg::Fetched(send_message(message).await) }
            });
        }

        Msg::Fetched(Ok(response_data)) => {
            ...
        }

        Msg::Fetched(Err(fetch_error)) => {
           ...
        }
    }
}

async fn send_message(new_message: String) -> fetch::Result<shared::SendMessageResponseBody> {
    Request::new(get_request_url())
        .method(Method::Post)
        .json(&shared::SendMessageRequestBody { text: new_message })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
```

Individual parts with explanations:

---

1.

```rust
pub enum Msg {
    ...
    SendRequest,
    Fetched(fetch::Result<shared::SendMessageResponseBody>),
}
```

`fetch::Result` is just an alias for `Result<T, FetchError>`. `T` is a custom response data type (`shared::SendMessageResponseBody` in our case).

---

2.

```rust
Msg::Fetched(Ok(response_data)) => {
    ...
}

Msg::Fetched(Err(fetch_error)) => {
    ...
}
```

`fetch::Result` is the `enum` with variants `Ok(T)` and `Err(FetchError)`. We can handle each variant by a dedicated `match` arm to eliminate nesting and improve readability.

---

3. 

```rust
Msg::SendRequest => {
    orders.skip().perform_cmd({
        let message = model.new_message.clone();
        async { Msg::Fetched(send_message(message).await) }
    });
}
```

- `skip()` isn't required, but we know that we don't modify `Model` at all so we can tell Seed that it doesn't have to rerender page - i.e. it can _skip_ rendering. `.skip()` is just a simple performance optimization.
- `async` functions/blocks are executed sometime in the future. That's why they often accept only owned values. In our case we need to clone `model.new_message` because it's possible that it will be mutated before our `async` block is executed and compiler doesn't allow this potentially harmful behavior.
- `async` blocks are basically `Future`s.
- Stable Rust supports only `async` blocks and functions. `async` closures aren't supported yet.
- `orders.perform_cmd` expects a `Future` as the argument. Then it executes the `Future` (by converting to Javascript `Promise`) and invokes app's `update` function with the `Msg` returned from the `Future` (if any).
- `send_message` is an `async` function - it returns a `Future` so we have to use `.await` to "unwrap" its inner value to match the type defined in `Msg::Fetched` - `fetch::Result<shared::SendMessageResponseBody>`.
- We can't make `update` async and just "await" the async operations because it would block the render loop and GUI would be frozen until the awaited `Future/Promise` is resolved.

---

4.

```rust
async fn send_message(new_message: String) -> fetch::Result<shared::SendMessageResponseBody> {
    Request::new(get_request_url())  // Prepare the request to the selected URL.
        .method(Method::Post)   // POST (default is GET)
        .json(&shared::SendMessageRequestBody { text: new_message })?   // Serialize payload to JSON. Serialization can fail and return `FetchError`.
        .fetch()   // Send the request.
        .await?   // Wait for the response. Request can fail and return `FetchError`.
        .check_status()?  // Make sure the response status is 2xx. Otherwise return `FetchError`.
        .json()   // Deserialize JSON to the required type. Rust is clever enough to know that it should deserialize to the return value type wrapped in `Result` - `shared::SendMessageResponseBody`.
        .await  // Wait for deserialization. It can fail and return `FetchError`.
}
```

See comments in the code above.
_Note:_ `?` means [early return](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=early,return#a-shortcut-for-propagating-errors-the--operator) on error.

---

## Example B

Let's look at the code from the previous chapter:

```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        ....
        .perform_cmd(async { 
            Msg::AuthConfigFetched(
                async { fetch("/auth_config.json").await?.check_status()?.json().await }.await
            )
        });

...

#[derive(Deserialize)]
struct AuthConfig {
    domain: String,
    client_id: String,
}

...

enum Msg {
    ...
    AuthConfigFetched(fetch::Result<AuthConfig>),
    
...

fn update(...) {
    match msg {
        ...
        Msg::HideMenu => {
            ...
        },
        Msg::AuthConfigFetched(Ok(auth_config)) => model.auth_config = Some(auth_config),
        Msg::AuthConfigFetched(Err(fetch_error)) => error!("AuthConfig fetch failed!", fetch_error),
        ...
    }
```

The structure is pretty similar to the previous `Example A`, so let's focus only on this part:

```rust
.perform_cmd(async { 
    Msg::AuthConfigFetched(
        async { fetch("/auth_config.json").await?.check_status()?.json().await }.await
    )
});
```

`fetch` function has this type: 
```rust
pub async fn fetch<'a>(request: impl Into<Request<'a>>) -> Result<Response>
```
It means `fetch` is basically a shortcut for `Request::new(...).fetch()` (see the previous example).

And `impl Into<Request<'a>>` allows us to pass different items as the argument. `From` for `Request` is currently implemented for:

```rust
impl<'a, T: Into<Cow<'a, str>>> From<T> for Request<'a> {
    ...
} // => it allows to pass `String`, `&str`, `Cow<str>`, etc.

impl<'a> From<Url> for Request<'a> {
    ...
}
```

---

## References

I hope you learned something about `fetch` and `async/.await`. 

Other links related to this topic:
- [Fetch example](https://github.com/seed-rs/seed/tree/master/examples/fetch) in the Seed repo.
- [Fetch documentation](https://docs.rs/seed/0.7.0/seed/browser/fetch/index.html) on `docs.rs`.
- [Async book](https://rust-lang.github.io/async-book/).

---

We'll return back to Time Tracker app and we'll try to explore [Slash GraphQL](https://dgraph.io/slash-graphql) in the next chapter.
