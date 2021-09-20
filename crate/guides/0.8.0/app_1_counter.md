# App 1: Counter

> When you click the button, the number is incremented.

[Live Demo](https://seed-app-counter.netlify.app/) |  [Repository](https://github.com/MartinKavik/seed-app-counter) |  [Playground](https://ide.play-seed.dev/?github=MartinKavik/seed-app-counter)

The counter example is the default example in the [basic quickstart](https://github.com/seed-rs/seed-quickstart), so you don't have to modify code in this tutorial at all.

Below is the entire code (`/src/lib.rs` content) without comments and extra items to satisfy linters:

```rust
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

struct Model { counter: i32 }

enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
```

You'll learn about individual parts (`Model`, `update`, etc.) in the next chapters. If you want to zoom out a bit before we jump into the rabbit hole, I recommend to read something about [The Elm Architecture (TEA)](https://guide.elm-lang.org/architecture/).
