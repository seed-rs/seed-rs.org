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
 model struct. When we call `Model::default()`, it initializes with these values. We could 
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
you pass to `App::builder()` describes how the state should change, upon
receiving each type of message. It's the only place where the model is changed. It accepts a message 
and a model as parameters, and returns an `Update` struct. `Update` contains `ShouldRender` and `Effect`
enums. `ShouldRender` and its variants are imported in the prelude, 
and has variants of 
`Render` and `Skip`. Render triggers a rendering update and will be used in 
most cases. `Skip` updates the model without triggering a render and is useful in animations.
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
[todoMVC example](https://github.com/seed-rs/seed/tree/master/examples/todomvc):
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

[TODO]: # (This section below is unclear, please improve me)

The third parameter of the update function implements the  
[Orders](https://docs.rs/seed/latest/seed/app/orders/trait.Orders.html)
 trait, imported in the prelude.
It has four methods, each defining an update behavior:

- `render`: Rerender the DOM, based on the new model. If `orders` is not used for a branch, it
is used.
- `skip`: Update the model without re-rendering
- `send_msg`: Update again, with a new message, the only parameter to this method
- `perform_cmd`: Perform an asynchronous task, like pulling data from a server. Its parameter
is a `Future`, ie `Future<Item = Ms, Error = Ms> + 'static`.

For an example of how to use orders, see the 
[orders example](https://github.com/seed-rs/seed/blob/master/examples/orders/src/lib.rs).

As with the model, only one update function is passed to the app, but it may be split into 
sub-functions to aid code organization.


## View
See the [view section](https://seed-rs.org/guide/view) for details.


## Initializing
To start your app, call the `App::builder` method, which creates an 
[Builder struct](https://docs.rs/seed/0.6.0/seed/app/builder/struct.Builder.html) struct. 
It has the the following optional methods:

- `before_mount` - Specify a function which allow you to 
select the HTML element where the app will be mounted and how it'll be mounted.
- `after_mount` - Used to initialize the model, and provide initial URL handling.
- `routes` - used to specify routing function. See the Routing section for details.
- `window_events` Registers a function which decides how window events will be handled.
- `sync` - Registers a sync function. (Fill out)

And the following mandatory one:
- `build_and_start` - run at the end, to initialize the app.

You can can chain the following optional methods:

- `.mount()` to mount in an element other than the one with id `app`.
- `.routes(routes)` to set a HashMap of landing-page routings, used to initialize your 
state based on url (See the `Routing` section)
- `.window_events(window_events)`, to set a function describing events on the `Window`. (See the `Events` section)

And must must complete with the method `.build_and_start();`.

The `App::builder` call must be wrapped in a function with the `#[wasm_bindgen(start)]` invocation.

Example using a custom mount point:
```rust
fn before_mount(_url: Url) -> BeforeMount {
   BeforeMount::new()
       .mount_point("main")
       .mount_type(MountType::Takeover)
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .before_mount(before_mount)
        .build_and_start();
}
```

This will render your app to the element holding the id you passed; in the case of this example,
"main". The only part of the web page Seed will interact with is that element, so you can
use other HTML not part of Seed, or other JS code/frameworks in the same document.

Example of using an `after_mount` function:
```rust
fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    AfterMount::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
```

`AfterMount` has the following fields:
    - `model`: The initial model
    - `url_handling`: A [Urlhandling](https://docs.rs/seed/latest/seed/app/builder/after_mount/enum.UrlHandling.html)  enum, which has 
    variants `PassToRoutes`: default with `Init::new()`),
    and `None`


`AfterMount::default()` covers the most common use-cases, where the model is initialized with its 
 `default::Default` implementation. (This is also true if we don't use the `.after_mount()` method.
 You can pass a different model by using `after_mount::new(model)`, where `model` here is your model.
 
 
Example, with `route` and `window_events`, described in the Routing and Misc sections of this guide
respectively:
```rust
#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .routes(routes)
        .window_events(window_events)
        .build_and_start();
}
```
