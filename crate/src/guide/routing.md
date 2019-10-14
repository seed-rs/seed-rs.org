# Routing
Seed includes flexible routing, inspired by 
[React-Reason](https://github.com/reasonml/reason-react/blob/master/docs/router.md): 
You can trigger state changes that update the address bar,
 and can be nagivated to/from using forward and back buttons. This works for landing-page
routing as well, provided your server is configured to support. See the
 [homepage](https://github.com/David-OConnor/seed/tree/master/examples/homepage) and
[todomvc](https://github.com/David-OConnor/seed/tree/master/examples/todomvc) examples.
  
Let's say our site the following pages:
a guide, which can have subpages, and a changelog, accessible by `http://seed-rs.org/changelog`,
`http://seed-rs.org/guide`, and `http://seed-rs.org/guide/3` (where 3 is the page we want) respectively. 
We describe the page by a `page`
field in our model, which is an integer: 0 for guide, 1 for changelog, and an additional
number for the guide page. An enum would be cleaner, but we don't wish to complicate this example.

## The basics

To set up the initial routing, pass a `routes` function describing how to handle
routing, to [App::build](https://docs.rs/seed/0.2.5/seed/struct.App.html#method.build)'s 
`routes` method.
```rust
fn routes(url: &seed::Url) -> Option<Msg> {
    if url.path.is_empty() {
        return Msg::ChangePage(0)
    }

    Some(match url.path[0].as_ref() {
        "guide" => {
            // Determine if we're at the main guide page, or a subpage
            match url.path.get(1).as_ref() {
                Some(page) => Msg::ChangeGuidePage(page.parse::<usize>().unwrap()),
                None => Msg::ChangePage(0)
            }
        },
        "changelog" => Msg::ChangePage(1),
        _ => Msg::ChangePage(0),
    })
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Init::new(Model::default()), update, view)
        .routes(routes)
        .finish()
        .run();
}
```

The simplest way to trigger routing is to set up an element with an `At::Href` attribute, who's
value contains a leading `/`, and corresponds to one of the routes defined in your `routes` function.
Clicking this will trigger routing, as defined in `routes`:

```rust
a!["Guide", attrs!{At::Href => "/guide"} ]
a!["Guide page 1", attrs!{At::Href => "/guide/1"} ]
```

The tag containing `Href` doesn't need to be an `a!` tag; any will work:

```rust
button!["Changelog", attrs!{At::Href => "/changelog"} ]
```


## More detail, and routing using events

Your `routes` function outputs the message that handles the routing as an `Option`, and accepts a 
[Url struct](https://docs.rs/seed/0.2.4/seed/routing/struct.Url.html)
describing the route, which routes has the following fields:
```rust
pub struct Url {
    pub path: Vec<String>,
    pub hash: Option<String>,
    pub search: Option<String>,
    pub title: Option<String>,
}
```
`path` contains the path heirarchy from top to bottom. For example, the `changelog` page above's path
is `vec![String::from("changelog")]`, representing `/changelog/`, and guide page 3's is 
`vec![String::from("guide"), 3.to_string()]`, representing `/guide/3/`. It's likely all you'll need.
The other three properties aren't as common; `hash` describes text after a `#`; `search` describes
text after a `?`, but before `#`, and title is a descriptive title, unimplemented in current web browsers, but may
see use in the future.

To trigger routing from events, instead of using `At::Href`, include logic like this in the `update` function:
```rust
#[derive(Clone)]
enum Msg {
    RoutePage(u32),
    RouteGuidePage(u32),
    ChangePage(u32),
    ChangeGuidePage(u32),
}

fn set_guide_page(guide_page: Page, model: &mut Model) {
    model.page = Page::Guide;
    model.guide_page = guide_page;
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RoutePage(page) => {
            seed::push_route(vec![page]);
            orders.skip().send_msg(Msg::ChangePage(page))
        },
        Msg::RouteGuidePage(guide_page) => {
            seed::push_route(vec!["guide", guide_page]);
            orders.skip().send_msg(Msg::ChangeGuidePage(guide_page))
        },
        // This is separate, because nagivating the route triggers state updates, which would
        // trigger an additional push state.
        Msg::ChangePage(page) => model.page = page,
        Msg::ChangeGuidePage(guide_page) => Render(Model {guide_page, page: Page::Guide, ..model}),
        Msg::ChangeGuidePage(guide_page) => {
            model.guide_page = page;
            model.page = Page::Guide;
        }
    }
}
```

Notice how the `Route` messages above call [seed::push_route](https://docs.rs/seed/0.2.5/seed/routing/fn.push_route.html), 
and the `Change` messages are called in the `routes` function, and are recursively called in the
update function. `push_route` accepts a single parameter: a `Url` struct, which you can create with a 
struct literal, or
 [seed::Url::new](https://docs.rs/seed/0.2.5/seed/routing/struct.Url.html#method.new). Alternatively,
  you can pass a `Vec<String>` / `Vec<&str>`, representing the path.

```rust
seed::push_route(
    seed::Url::new(vec!["myurl"])
        .hash("textafterhash")
        .search("textafterquestionmark")
)
```
 
When a page is loaded or browser naviation occurs (eg back button), Seed uses the `routes`
func you provided to determine which message to call. 

Notice how we keep ChangePage and RoutePage separate in our example. Do not
call `push_route` from one of these messages, or you'll end up with recusions/unwanted behavior:
 `ChangePage` in our example performs
the action associated with routing, while `RoutePage` updates our route history, then
recursively calls `ChangePage`. If you were to attempt this in the same message, each
browser navigation event would add a redundant route history entry, interfering with navigation. `

We call routing messages from in-app navigation events, like this:

```rust
h2![ simple_ev(Ev::Click, Msg::RoutePage(0)), "Guide" ]
```

Or programatically using lifecycle hooks:

```rust
    did_mount(move |_| {
        if model.logged_in {
            state.update(Msg::RoutePage(0))
        }
    })
```

To make landing-page routing work, configure your server so that all relevant paths towards the 
root or html file,
 instead of returning an error. The `serve.py` script
included in the quickstart repo and examples is set up for this. Once this is configured, intial 
routing on page load will work as expected: The page will initialize with the default state, then immediately 
update based on the message returned by the `routes` function.
