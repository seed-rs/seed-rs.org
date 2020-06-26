# App 3: Time Tracker

[Live Demo](https://seed-app-time-tracker.netlify.app/) |  [Repository](https://github.com/MartinKavik/seed-app-time-tracker)

## Intro

Let's write something more useful - a real Time Tracker with user accounts and full back-end integration! Step-by-step from motivations and design to backend and deploy.  

You'll learn:
- Fire & handle [Fetch](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) and [GraphQL](https://graphql.org/) requests.
- About [JSON Web Tokens](https://jwt.io/).
- To write modules/components.
- Advanced routing, link building and how to change a base path.
- To work with global state / context.
- To architect a large app.
- How to integrate [SASS](https://sass-lang.com/) and [Bulma](https://bulma.io/).

## Motivation

I work as a freelance Rust developer to pay my rent while I'm working on Seed. I send an invoice per x hours of work to my clients. 

And I want:
  - To know when I should send a new invoice - i.e . when the next _time block_ of x hours will be reached.
  - Mark previous time blocks as paid or unpaid.
  - Link a time block to the associated invoice.
I use(d) services like [toggl](https://toggl.com/) or [Clockify](https://clockify.me/) but the features mention above have been often missing and I don't need the most of built-in features.

I think this example will be easily customizable to meet your requirements. And the most of us use or at least know a time tracker so we won't need to explain some unfimiliar and complex logic and terminology.

The example should also demostrate how to implement basic auth logic - registration & login. 

_Tip:_ We should always identify what are the problems we want to resolve before we even start to think about our new app.

---

We have a rough idea what we want to build. Let's add more details in the next chapter.
