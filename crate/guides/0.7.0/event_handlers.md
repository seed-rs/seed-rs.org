# Event Handlers

Event handlers `ev`, `mouse_ev`, etc. represent HTML [EventListeners](https://developer.mozilla.org/en-US/docs/Web/API/EventListener) in the Seed world, e.g.:
```rust
button![
    model, 
    ev(Ev::Click, |_| Msg::Increment),
]
```
becomes
```html
<button>15</button>
```
where click on the `button` invokes your `update` function with `Msg::Icrement` as an argument.

# ev

`ev` is the basic event handler. The most common usage is:
```rust
ev(Ev::Click, |_| Msg::Increment)
```
## Event

The first argument (`Ev::Click`) is called _event_ or _trigger_.

It can be any `Ev` variant or you can write a custom one by `Ev::from("custom_event")`. Custom events may be useful in combination with custom elements.

## Callback

The second argument(`|_| Msg::Increment`) is called _callback_ or _handler_.

`ev`'s callback expects [web_sys::Event](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html) as an input. [Event](https://developer.mozilla.org/en-US/docs/Web/API/Event) is the most general iterface for all events. It's useful for calling methods like [preventDefault](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault):
```rust
ev(Ev::Click, |event| {
    event.prevent_default();
    Msg::LoginClicked
})
```

<details>
<summary>What is <code>web_sys</code>?</summary>

[web_sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate is basically a Rust interface for Web APIs. You'll see it in many examples and docs. It's automatically imported from `seed`. 
However the Seed's one doesn't cover all Web APIs - when the compiler can't find a method, but you are sure `web_sys` supports it, you'll have to enable corresponding `web_sys`'s features - see [Cargo.toml](https://github.com/seed-rs/seed/blob/a240eab1c69b21f9dbe07134f821546bdfbfb616/examples/user_media/Cargo.toml#L14-L21) in `user_media` example. You'll find required features in `web_sys` docs - e.g. [HtmlMediaElement](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlMediaElement.html).

</details>

You can return from callbacks:
1. `Msg`
```rust
ev(Ev::Click, |_| Msg::Clicked),
ev(Ev::from("data-loaded"), Msg::DataLoaded`) // `DataLoaded(web_sys:Event)`
```
2. `Option<Msg>`
```rust
ev(Event::Click, |_| Some(Msg::Clicked))
```
3. `()`
```rust
ev(Ev::Click, |event| log!("Clicked!", event)),
ev(Ev::Click, |event| {
    event.prevent_default();
    event.stop_propagation();
})
```

_Note_: TODO runtime error vs compile-time error

## Event Casting

There are many specialized event types like [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent) and [KeyboardEvent](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent). However we receive only `Event` in our `ev` callbacks - we need to cast the general `Event` to concrete event sub-type to use its associated methods:

```rust
dynref dyninto uncheckedinto
```



@TODO Mention window event handlers? Are they in TODOMVC? Auth? Or link to subscribe example?

