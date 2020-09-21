# App 3: Time Tracker

_NOTE:_ It's **WIP**! Routing, Auth0 authentication and GraphQL queries + Slash GraphQL backend are done (at the time of writing). The most of views, authorization, mutations and SCSS compilation are todos. But It should already give you an idea how to design, architect and write a relatively large app.

---

[Live Demo](https://seed-app-time-tracker.netlify.app/) |  [Repository](https://github.com/MartinKavik/seed-app-time-tracker)
 - Email address: `john@example.com`
 - Password: `Password1`

## Intro

Let's write something more useful - a real Time Tracker with user accounts and full backend integration! Step-by-step from motivations and design to backend and deploy.  

You'll learn:
- To architect a relatively large app (frontend + backend).
- To write Seed modules/components.
- Advanced routing, link building and how to change a base path.
- To work with global state / context.
- Call Javascript functions from the Rust world.
- When to use Seed's notify/subscribe mechanism.
- Fire & handle [Fetch](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) and [GraphQL](https://graphql.org/) requests.
- About [JSON Web Tokens](https://jwt.io/).
- How to integrate [SASS](https://sass-lang.com/), [Bulma](https://bulma.io/) and [Font Awesome](https://fontawesome.com/).
- To define [Slash GraphQL](https://dgraph.io/slash-graphql) backend.
- Setup and integrate [Auth0](https://auth0.com/).

## Motivation

I work as a freelance Rust developer to pay my rent while I'm working on Seed. I send an invoice per x hours of work to my clients. 

And I want:
  - To know when I should send a new invoice - i.e . when the next _time block_ of x hours will be reached.
  - Mark previous time blocks as paid or unpaid.
  - Link a time block to the associated invoice.
I use(d) services like [toggl](https://toggl.com/) or [Clockify](https://clockify.me/) but the features mention above have been often missing and I don't need the most of built-in features.

I think this example will be easily customizable to meet your requirements. And the most of us use or at least know a time tracker so we won't need to explain some unfamiliar and complex logic and terminology.

The example should also demonstrate how to implement basic auth logic - registration & login. 

_Tip:_ We should always identify what are the problems we want to resolve before we even start to think about our new app.

---

We have a rough idea what we want to build. Let's add more details in the next chapter.
