# Init

Counter example part:

```rust
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
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

- `init` function is called only once, when your app started.

- The main purpose of this function is to create a `Model` instance.

In the Counter example, `init` parameters don't have names (there is only `_` as a placeholder) . It signals to readers, the compiler and linters that we don't use those parameters in the function body. (_Note:_ We can also add only prefix `_` before the names (`_url` and `_orders`) instead, but it's relatively easy to overlook `_` and Clippy can be sad about that because it doesn't ignore prefixed names (unlike the compiler - it ignores them).)

## Parameter `url: Url`

It's very common that some fields in your `Model` depend on the current Url.
You'll often write the code similar to:
```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    ...
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url),
        ...
```
That's why we decided to add parameter `url` - for your comfort (aka _developer experience_) - you don't have to go through the documentation every time you need to get `Url` and introduce side-effects or slow JS calls into your code-base by trying to get it directly from the browser.

## Parameter `orders: &mut impl Orders<Msg>`

It's the way how you are "giving orders" to Seed. Do you want to send an HTTP request? Do you want to subscribe to Url changes? Do you want to do something after 5 seconds? Well, use `orders`.

`orders` has many useful methods, we will discuss them in other chapters. Example usage:

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

Well, let me explain why it hasn't got a simpler type instead. There are possible options: 

1. Without `&mut`
   - `fn init(_: Url, orders: impl Orders<Msg>) -> Model`
   
   - `orders` contains data like a side-effect queue. So when you call e.g. `orders.perform_cmd(.. fetch ..)` you basically modifies the queue. We can move the queue into a wrapper with [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value), but it isn't idiomatic or explicit enough and it would be slower and more error-prone.


1. Without `<Msg>`
   - `fn init(_: Url, orders: &mut impl Orders) -> Model`
   
   - The compiler and IDEs need help - without it they don't know if our HTTP response handlers return the expected `Msg` type; that we want to send the correct `Msg`; or they can't show you possible options in autocomplete lists.
   
   - We can hide `<Msg>` by some magic provided by [Any](https://doc.rust-lang.org/std/any/), but you are basically trying to remove static types from Rust.. it's not idiomatic of course and very error-prone. Also there will be multiple `Msg`s in your app and each would have its own `orders` - so it makes sense to distinguish them explicitly.

1. Without `impl`
   - `fn init(_: Url, orders: &mut Orders<Msg>) -> Model`

   - `orders` contains a reference to `App` instance - it's required by some `orders` methods and there are some cases when it's useful for users, too. However struct `App` requires multiple type parameters. And we don't want to "leak" them into `orders` - it would look like `orders: &mut Orders<Msg, Model, Vec<Node<Msg>>>`. So `Orders` isn't a specific type but a [trait](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior). And those extra `App` types are hidden in `Orders`'s [associated types](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types) with [impl](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters) help. (_Note:_ We can't hide also type parameter for `Msg` because it would cause cumbersome "type acrobatics" in your components.)

</details>

---

_Try_: Replace `Model::default()` with `10`, wait until the compilation is complete and then reload the browser tab with your Counter example and see the result.

---
