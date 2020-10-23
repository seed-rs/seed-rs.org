# Start

Counter example part:

```rust
// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
```

<details>
<summary>Example from a production app (this website)</summary>

```rust
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
```

</details>

---

- `start` is called when your Rust code is loaded by the browser.

- Its main purpose is to start the app.

## Attribute `wasm_bindgen`

[wasm_bindgen](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html) is basically a bridge between Rust and Javascript.

Functions tagged with `#[wasm_bindgen]` can be called from JS. There are many settings and use-cases described in [the official docs](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/index.html), however the interesting option for us is [start](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/start.html):
> When attached to a pub function this attribute will configure the start section of the wasm executable to be emitted, executing the tagged function as soon as the wasm module is instantiated.

## `start` function signature

> `pub fn start()`

The official docs are a bit wrong - you can return from the function tagged with `#[wasm_bindgen(start)]` also all types that `wasm_bindgen` can transform to a consumable form for JS => you can return everything that implements [FromWasmAbi](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/convert/trait.FromWasmAbi.html). See e.g. [`Box<[JsValue]>`](https://github.com/seed-rs/seed/blob/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/examples/update_from_js/src/lib.rs#L76) in `update_from_js` example. It's one of the reasons why this function is named `start` and not `main` - to force you to think differently about it.

_Note:_ `start` has to be `pub` because it's called from the outside (i.e. from the JS world).

## How to invoke `start`

`start` is invoked when the WASM module (Rust code) is loaded. The simplest code that loads the `*.wasm` file:
```html
<script type="module">
    import init from '/pkg/package.js';
    init('/pkg/package_bg.wasm');
</script>
```
You can find it in your [index.html](https://github.com/MartinKavik/seed-app-counter/blob/master/index.html).

`pkg` folder and its content is generated by [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/introduction.html). `wasm-pack` is a CLI tool that takes your Rust code and generates WASM file, Javascript/Typescript files and some other assets for you. These JS/TS files contain types and scripts to handle communication between your app and ugly JS/HTML world.
`package.js` is the most important one because it loads the app (`package_bg.wasm`).

## `App::start`

> `App::start("app", init, update, view);`

`App::start` is the method that finally starts our Seed app. 
1. It mounts the app into the chosen root element. (We'll talk about it more later).
1. Does some low-level app initialization - setups listeners, loads base path for routing, enable panic logging to the console, etc.
1. Calls your `init` function.
1. Render the app for the first time.
1. Returns the `App` instance. It's useful when you need to setup some callbacks as soon as possible (see example [update_from_js](https://github.com/seed-rs/seed/blob/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/examples/update_from_js/src/lib.rs#L77-L79)).

<details>
<summary><code>App::start</code> signature explanation</summary>

I can imagine `App::start` signature could be difficult to grasp for beginners so there is an explanation. Commented code from Seed's [app.rs](https://github.com/seed-rs/seed/blob/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/src/app.rs):

```rust
// `App` has generic type parameters `Ms`, `Mdl` and `INodes`.
// `Ms` represents your `Msg`, `Mdl` your `Model` 
// and `INodes` the output value from your `view`. 
impl<Ms, Mdl, INodes> App<Ms, Mdl, INodes>

// We use `where` for better readability. An alternative would be:
// `impl<Ms, Mdl, INodes: IntoNodes<Ms> + 'static> App<Ms, Mdl, INodes>`
where

    // The most of things has to be `'static` because it mitigates 
    // the most of lifetime pains for Seed users but primarily we can't pass 
    // references into JS closures - and we need JS closures a lot,
    // especially in the Seed's core for all listeners and similar stuff.
    //
    // _Note_: I had problems to understand what is `'static` 
    // when I was learning Rust => it DOESN'T mean that the value is in memory 
    // for the entire program/app lifetime. It means it's in memory 
    // as long as necessary and we can pass it as we want
    // and we can store it inside structs without problems - in other words: 
    // it's basically everything, except references. 
    // There are only 2 exceptions:
    // 1. `&'static str` (aka string literals) - they are hardcoded strings
    // in your binary so you don't have to do any runtime memory allocations.
    // 2. `static NUM: i32 = 18;` - static global values. They live 
    // for the entire app lifetime.
    INodes: IntoNodes<Ms> + 'static,
{
    pub fn start(

        // You have to pass something that implements trait `GetElement` as `root_element`.
        // `impl` just hides the concrete type so it often makes the code MUCH more readable
        // and you don't have to introduce generic type parameters. 
        // (More about `GetElement` later.)
        root_element: impl GetElement,

        // There is `impl FnOnce` instead of `fn` so you can pass also a closure as `init`:
        // `App::start("app", |_, _| Model::default(), update, view)`
        //
        // It's `FnOnce + 'static` so the closure can close almost everything:
        // `let something = Everything::new();`
        // `App::start("app", move |_, _| Model { data: something }, update, view)`
        //
        // `OrdersContainer<..>` is a concrete type that implements `Orders`.
        // _Note:_ You shouldn't use `OrdersContainer` directly in your code.
        init: impl FnOnce(Url, &mut OrdersContainer<Ms, Mdl, INodes>) -> Mdl + 'static,

        // `update` is also `FnOnce` although it should be `Fn` because it's called multiple times.
        // There is a little trick: It also implements trait `Clone`, 
        // so it can be "casted" to `Fn` under the hood with the code like:
        // `let i_am_Fn = |argument| i_am_FnOnce.clone()(argument)`
        // That way both the compiler and users are happy - compiler is ok with multiple calls,
        // and `FnOnce + Clone` is the most convenient API for users. 
        // _Note:_ `Clone` is cheap because in the most cases `update` 
        // is `fn` that always implements `Copy` (`Copy` is just a mark for cheap `Clone`). 
        update: impl FnOnce(Ms, &mut Mdl, &mut OrdersContainer<Ms, Mdl, INodes>) + Clone + 'static,
        
        // Rust compiler isn't (yet?) able to compile `impl` in some places 
        // so we have to use generic type parameter `INodes` here.
        view: impl FnOnce(&Mdl) -> INodes + Clone + 'static,

        // `Self` represents `App<...>`.
    ) -> Self { ...
```

</details>

## Mounting

App mounting = app initialization + the first render to the selected HTML element (aka root element).

The first argument in `App::start` method will be your root element:
```rust
App::start("app", init, update, view);
```
It can be everything that implements [GetElement](https://github.com/seed-rs/seed/blob/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/src/app/get_element.rs):
- `&str` that represents an element `id`. The app panics if it can't find the element. It's the standard way.
- [web_sys::Element](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html)
- [web_sys::HtmlElement](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlElement.html)

`Element` and `HtmlElement` support is useful when you want to find the element by class or tag name - see e.g. [get_elements_by_tag_name](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html#method.get_elements_by_tag_name) - it would look like:
```rust
#[wasm_bindgen(start)]
pub fn start() {
    // You have to enable panic messages forwarding
    // to console log. The same line is also in `App::start`,
    // but it's too late for the `.expect` below.
    //
    // _Note:_ For Seed <= 0.7.0, you have to add `console_error_panic_hook`
    // into dependencies in your `Cargo.toml` or use this commit:
    // `seed = { git = "https://github.com/seed-rs/seed", rev = "0a538f0" }`
    console_error_panic_hook::set_once();

    let root_element = document()
        .get_elements_by_tag_name("section")
        .item(0)
        .expect("`section` as a root element");

    App::start(root_element, init, update, view);
}

```

Once the root element is successfully found, its content is replaced with the rendered app. So it's NOT RECOMMENDED to mount the app into `body`, because `body` often contains user's scripts and because browser extensions/plugins like to inject their scripts into `body`. (It's a general rule for all frontend frameworks).


## How to write a good `start`

- In the most cases it's a one-liner.

- Don't forget to add `console_error_panic_hook::set_once();` at the top of the `start` body if there is some code above the `App::start(..)` call.

- If you really need to pass something to JS, make it as simple as possible. And keep in mind that it'll introduce a lot of boilerplate and error-prone code. See example [update_from_js](https://github.com/seed-rs/seed/tree/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/examples/update_from_js).

- When you need to write some helpers, respect the rule *"children below the parent"* as always.

