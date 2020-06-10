# App 2: TodoMVC

## Intro

Well, I hope your head didn't explode due to the amount of information in the previous example. You should be ready for a real step-by-step tutorial now - how to write a classic [TodoMVC example](http://todomvc.com/) from scratch.

You'll learn:
- To design a good `Model` based on specifications. 
- Routing.
- Window event handling. 
- How and when to use Element References and Keys.
- How to use [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage). 

There are two ways to write Seed apps from scratch:

1. Design `Model` and `Msg` first and then write other app parts.
   - It's the cleanest and preferable way. 

1. Write `view` first and then other items.
   - It's useful for websites where the `Model` will be simple; for prototypes; and if you like rewritting a lot. Also it's useful when you are forced to do something like [Scrum](https://zenkit.com/en/blog/scrum-101-an-introduction-to-scrum-project-management/) and you want to show at least something on the demo.

Let's begin!

## Specifications

There are [specs](https://github.com/tastejs/todomvc/blob/master/app-spec.md#functionality) in the official TodoMVC repo. They'll help us to design `Model` in the next chapter.

![TodoMVC screen](/static/images/todomvc_screen.png)










