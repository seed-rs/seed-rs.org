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
where click on the `button` invokes your `update` function with `Msg::Increment` as an argument.

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

`ev`'s callback expects [web_sys::Event](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html) as an input. [Event](https://developer.mozilla.org/en-US/docs/Web/API/Event) is the most general interface for all events. It's useful for calling methods like [preventDefault](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault):
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
ev(Ev::from("data-loaded"), Msg::DataLoaded`) // `enum Msg { DataLoaded(web_sys:Event) }`
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

_Note_: The app panics when you try to return unsupported type. This runtime error will be turned into the compile-time one, once the required Rust feature is stabilized.

## Event Casting & Helpers

There are many specialized event types like [MouseEvent](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent) and [KeyboardEvent](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent). However we receive only `Event` in our `ev` callbacks - we need to cast the general `Event` to concrete event sub-type to use its associated methods:

```rust
ev(Ev::Click, |event| {
    let mouse_event: web_sys::MouseEvent = event.unchecked_into();
    log!(mouse_event.ctrl_key());
}),
ev(Ev::Click, |event| {
    IF!(event.unchecked_into::<web_sys::MouseEvent>().shift_key() => Msg::Increment)
})
```

`unchecked_into` is one of the [casting methods](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html). 

_Note:_ Syntax `::<web_sys::MouseEvent>` is known as a [turbofish](https://github.com/jplatte/turbo.fish).

To eliminate error-prone boilerplate introduced by casting, there are some `ev`-related functions that cast the event before they call your callback:
- `keyboard_ev` casts `Event` to [KeyboardEvent](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html)
- `mouse_ev` => [MouseEvent](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.MouseEvent.html)
- `touch_ev` => [TouchEvent](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.TouchEvent.html)
- `drag_ev` => [DragEvent](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.DragEvent.html)
- `pointer_ev` => [PointerEvent](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.PointerEvent.html)

There is one exception: `input_ev`. It doesn't cast `Event` into [InputEvent]() - it tries to get the value directly from the element, because in the most cases you just want to know the changed value and `InputEvent` doesn't contain it for some reasons. Example:
```rust
enum Msg {
    EmailChanged(String),
}
...

input![
    attrs! {
        At::Value => model.email
    },
    input_ev(Ev::Input, Msg::EmailChanged)
],
```

_Note:_ All helpers panic if it's not possible to cast the event to the required event type.

## Window / Document Events

They will be discussed in other chapters, however you can take inspiration from examples [window_events](https://github.com/seed-rs/seed/blob/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/examples/window_events/src/lib.rs#L43-L48) and [subscribe](https://github.com/seed-rs/seed/blob/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/examples/subscribe/src/lib.rs#L15-L18).

## Future

`*ev` functions will be replaced with `E` struct to eliminate boilerplate and improve safety and readability. You can find API drafts in [this issue](https://github.com/seed-rs/seed/issues/331#issuecomment-590956553).

