# Comparisons to React and Vue code

On this page, we'll show equivalent code snippets in Seed, and other frameworks. For now, we
 include examples from `React` and `Vue`. 
The [TodoMVC example](https://github.com/seed-rs/seed/tree/master/examples/todomvc) can be used to 
compare to [its implementation in other frameworks](http://todomvc.com/).

Note that there are multiple ways to manage state in React, we've picked one where we store state
in the top-level component, and use functional components thereafter.
A closer structure match would be using it coupled with Redux. The Context API's an additional
way to handle it. We're also using Typescript.

## A simple template, ready for state management

## React

```typescript
import * as React from "react"
import * as ReactDOM from "react-dom"

interface MainProps {}
interface MainState {
    val: number
}

class Main extends React.Component<MainProps, MainState> {
    constructor(props) {
        super(props)

        this.state = {
            val: 0
        }

        this.increment = this.increment.bind(this)
    }
    
    increment() {
        this.setState({val: this.state.val + 1})
    }
    
    render() {
        return (
            <button onClick={() => this.state.increment()}>
                {"Hello, World × " + this.state.val}
            </button>
        )   
    }
}

ReactDOM.render(<Main />, document.getElementById("app"))
```


## Seed
From the Seed quickstart repo
```rust
#[macro_use]
extern crate seed;
use seed::prelude::*;

struct Model {
    pub val: i32,
}

impl Default for Model {  // In this case, we could derive `Default` instead.
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.val += 1,
    }
}

fn view(model: &Model) -> impl View<Msg> {
    button![
        simple_ev(Ev::Click, Msg::Increment),
        format!("Hello, World × {}", model.val)
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}
```

## Attributes and styles

## Events

## Reusable UI items

## HTTP Requests (todo)

## Configuration files and tooling (todo)