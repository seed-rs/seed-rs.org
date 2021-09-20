# Init

Counter example part:

```rust
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app starts
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}
```

<details>
<summary>Example from a production app (this website)</summary>

```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    let guides = guide::guides();
    let mut selected_seed_version = DEFAULT_SEED_VERSION;

    Model {
        base_url: url.to_base_url(),
        page: Page::init(url, &guides, &mut selected_seed_version),
        selected_seed_version,
        guide_list_visibility: Hidden,
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
        guides,
        search_query: String::new(),
        matched_guides: Vec::new(),
        mode: load_config().mode,
    }
}
```

</details>

---

- The main purpose of this function is to create a `Model` instance.

- The `init` function is only called when your app starts.

In the Counter example, `init` parameters don't have names, instead using `_` as a placeholder. This lets readers, linters, and the compiler know that we don't use those parameters in the function body. 

_Note:_ We can also prefix `_` before variable names (e.g. `_url` and `_orders`), but it's relatively easy to overlook `_`. Clippy also complains about that because it doesn't ignore prefixed names, unlike the compiler.

## Parameter `url: Url`

It's very common for some fields in your `Model` depend on the current URL.
You'll often write the similar to the example below:
```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    ...
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url),
        ...
```
That's why we decided to add the `url` parameter - for your comfort (aka _developer experience_). You don't have to go through the documentation every time you need to get `Url` and introduce side effects or slow JS calls into your codebase by trying to get it directly from the browser.

## Parameter `orders: &mut impl Orders<Msg>`

Think of this parameter as "giving orders" to Seed. Do you want to send an HTTP request? Do you want to subscribe to URL changes? Do you want to do something after 5 seconds? Well, use `orders`.

`orders` has many useful methods, and we will discuss them in later chapters. Here's a usage example:

```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url))
        .stream(streams::window_event(Ev::Resize, |_| Msg::OnResize));
    ...
```

<details>
<summary>Why does <code>orders</code> have such a weird type??</summary>

Well, let me explain why it doesn't have a simpler type. There are possible options: 

1. Without `&mut`
   - `fn init(_: Url, orders: impl Orders<Msg>) -> Model`
   
   - `orders` contains data like a queue, and calling something like `orders.perform_cmd(.. fetch ..)` modifies the queue. We can move the queue into a wrapper with [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value), but that isn't idiomatic or explicit enough, would be slower, and is more error-prone.


1. Without `<Msg>`
   - `fn init(_: Url, orders: &mut impl Orders) -> Model`
   
   - The compiler and IDEs need help - they can't infer that our HTTP response handlers return the expected `Msg` type, and they can't show you possible options in autocomplete lists without it.
   
   - We can hide `<Msg>` through some magic provided by the [any](https://doc.rust-lang.org/std/any/) module in the standard library, but doing that would be like trying to remove static types from Rust. It's not idiomatic of course and very error-prone.

1. Without `impl`
   - `fn init(_: Url, orders: &mut Orders<Msg>) -> Model`

   - `orders` contains a reference to the `App` instance - it's required by some `orders` methods and there are some cases where it's useful for users, too. The `App` struct, however, requires multiple type parameters, and we don't want to "leak" them into `orders` - doing this would look something like `orders: &mut Orders<Msg, Model, Vec<Node<Msg>>>`. As a result, `Orders` isn't a specific type but a [trait](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior), and those extra `App` types are hidden in `Orders`'s [associated types](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types) with [impl](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters) help. (_Note:_ We also can't hide type parameter for `Msg` because it would cause cumbersome "type acrobatics" in your components.)

</details>

## How to write a good `init` function

- Your `init` function should be short and simple - the main goal is to just create a new `Model` instance. Furthermore, it blocks the app - try to invoke time-consuming operations in other functions (especially in `update` function; you'll learn about `update` in next chapters) when the app is rendered and the user is happy that he sees at least some content.

- When you need to write some helpers, respect the *"children below the parent"* rule - write helpers below the `init` function, and you are using some helpers in the `update` function, move them below it.

---

_Try_: Replace `Model::default()` with `10`, wait until the compilation is complete and then reload the browser tab with your Counter example and see the result.

---
