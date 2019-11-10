# App structure

## Model
Each app must contain a model [struct]( https://doc.rust-lang.org/book/ch05-00-structs.html), 
which contains the app’s state. It must should contain 
[owned data](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html). References
with a static [lifetime](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) work,
but may be more difficult to work with. Example:

```rust
struct Model {
    count: i32,
    what_we_count: String
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into()
        }
    }
}
```
 
In this example, we initialize using Rust’s `Default` trait, in order to keep the initialization code by the
 model struct. When we call `Model.default()`, it initializes with these values. We could 
 also initialize it using a constructor method, or a struct literal. Note the use of `into()` 
 on our `&str` literal, to convert it into an owned `String`.
 
The model holds all data used by the app, and will be replaced with updated versions when the data changes.
Use owned data in the model; eg `String` instead of `&'static str`. The model may be split into sub-structs to organize it – 
this is especially useful as the app grows:
 
```rust
struct FormData {
    name: String,
    age: i8,
}

struct Misc {
    value: i8,
    descrip: String,
}

struct Model {
    form_data: FormData,
    misc: Misc
}
```

## Update
The Message is an [enum]( https://doc.rust-lang.org/book/ch06-00-enums.html) which 
categorizes each type of interaction with the app. It must implement `Clone`, and its 
fields may hold a value, or not.
We’ve abbreviated it as `Msg` here for brevity. If you're not familiar with enums,
think of one as a set of options; in other languages, you might use an integer, or string 
for this, but an enum is explicitly limited in which values it can take. Example:

```rust
#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeDescrip(String),  //  We could use &'static str here too.
}
```
 
The update [function]( https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) 
you pass to `seed::App::build(` describes how the state should change, upon
receiving each type of message. It's the only place where the model is changed. It accepts a message, 
and model as parameters, and returns an `Update` struct. `Update` contains `ShouldRender` and `Effect`
enums. `ShouldRender` and its variants are imported in the prelude, 
and has variants of 
`Render` and `Skip`. Render triggers a rendering update, and will be used in 
most cases. `Skip` updates the model without triggering a render, and is useful in animations.
`Effect` isn't exposed in the API: it's used internally to handle async events like
fetch requests. See the `Http requests` section for more info.

Example:
```rust
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::SetCount(count) => model.count = count,
    }
}
```

While the signature of the update function is fixed, and will usually involve a 
match pattern with an arm for each message, there
are many ways you can structure this function. Some may be easier to write, and others may 
be more efficient, or appeal to specific aesthetics. While the example above
it straightforward, this becomes important with more complex updates.

More detailed example, from the 
[todoMVC example](https://github.com/David-OConnor/seed/tree/master/examples/todomvc):
```rust
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClearCompleted => {
            model.todos = model.todos.into_iter()
            .filter(|t| !t.completed)
            .collect();
        },
        Msg::Destroy(posit) => {
            model.todos.remove(posit);
        },
        Msg::Toggle(posit) => model.todos[posit].completed = !model.todos[posit].completed,
        Msg::ToggleAll => {
            let completed = model.active_count() != 0;
            for todo in &mut model.todos {
                todo.completed = completed;
            }
        }
}
```

The third parameter of the update function is an 
[Orders](https://docs.rs/seed/0.3.4/seed/prelude/struct.Orders.html)
 struct, imported in the prelude.
It has four methods, each defining an update behavior:

- `render`: Rerender the DOM, based on the new model. If `orders` is not used for a branch, it
is used.
- `skip`: Update the model without re-rendering
- `send_msg`: Update again, with a new message, the only parameter to this method
- `perform_cmd`: Perform an asynchronous task, like pulling data from a server. Its parameter
is a `Future`, ie `Future<Item = Ms, Error = Ms> + 'static`.

For an example of how to use orders, see the 
[orders example](https://github.com/David-OConnor/seed/blob/master/examples/orders/src/lib.rs).

As with the model, only one update function is passed to the app, but it may be split into 
sub-functions to aid code organization.


## View
See the [view section](https://seed-rs.org/guide/view) for details.


## Initializing
To start your app, call the `seed::App::build` method, which takes the following parameters:

- An `init` function which accepts an initial routing, initial orders, and outputs 
an [Init struct](https://docs.rs/seed/0.4.1/seed/struct.Init.html) (imported in the prelude),
 wrapping the initial model.
- Your update function
- Your view function

You can can chain the following optional methods:

- `.mount()` to mount in an element other than the one with id `app`.
- `.routes(routes)` to set a HashMap of landing-page routings, used to initialize your 
state based on url (See the `Routing` section)
- `.window_events(window_events)`, to set a function describing events on the `Window`. (See the `Events` section)

And must must complete with the method `.build_and_start();`.

`.mount()` takes a single argument, which can be the id of the element you wish to mount in,
a `web_sys::Element`, or a `web_sys::HtmlElement`. Examples:
`seed::App::build(|_, _| Model::default(), update, view).mount(seed::body())`
`seed::App::build(|_, _| Model::default(), update, view).mount('a_div_id`)`

```
seed::App::build(|_, _| Model::default(), update, view).mount(
    seed::body().querySelector("section").unwrap().unwrap()
)
```

The `seed::App::build` call must be wrapped in a function with the `#[wasm_bindgen(start)]` invocation.

This will render your app to the element holding the id you passed; in the case of this example,
"main". The only part of the web page Seed will interact with is that element, so you can
use other HTML not part of Seed, or other JS code/frameworks in the same document.

Example, with optional methods:
```rust
#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Init::new(Model::default()), update, view)
        .mount("main")
        .routes(routes)
        .window_events(window_events)
        .build_and_start();
}
```

Example of using a standalone `init` function:
```rust
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Init<Model> {
    Init::new(Model::default())
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view)
        .build_and_start();
}
```

`Init` has the following fields:
    - `model`: The initial model
    - `url_handling`: A [Urlhandling](https://docs.rs/seed/0.4.1/seed/enum.UrlHandling.html)  enum, which has 
    variants `PassToRoutes`: default with `Init::new()`),
    and `None`
    - `mount_type`: A [MountType](https://docs.rs/seed/0.4.1/seed/enum.MountType.html)  enum, which has variants `Append`: default with `Init::new()`,
    Leave the previously existing elements in the mount alone. This does not make guarantees of
    elements added after the `App` has been mounted),
    and `Takeover`:  Take control of previously existing elements in the mount. This does not make guarantees of
    elements added after the `App` has been mounted. Note that existing elements in the DOM will 
    be recreated. This can be dangerous for script tags and other, similar tags.

`Init::new()` covers the most common use-cases of the `Init`, but pass an `Init` literal if you'd
like to use `url_handling` or `mount_type`. `UrlHandling` and `MountType` are imported in the prelude.

