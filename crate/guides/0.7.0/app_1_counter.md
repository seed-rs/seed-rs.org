# App 1: Counter

> When you click the button, the number is incremented.

[Live Demo](https://seed-app-counter.netlify.app/) |  [Repository](https://github.com/MartinKavik/seed-app-counter)

Counter example is the default example in the [basic quickstart](https://github.com/seed-rs/seed-quickstart) so you don't have to modify code at all.

The entire code (`/src/lib.rs` content) without comments and extra items to satisfy linters:

```rust
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

type Model = i32;

enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
```

You'll learn about individual parts (`Model`, `update`, etc.) in next chapters. And then we'll zoom out a bit and explain Seed app architecture. 
