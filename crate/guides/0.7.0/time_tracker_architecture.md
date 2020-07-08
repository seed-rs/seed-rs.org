# Architecture

We have an idea how the app should work because we've defined basic entities, user flows and technical requirements. Let's talk about architecture and hosting.

# Frontend

Our Time Tracker app is basically an ideal case for [SPA](https://en.wikipedia.org/wiki/Single-page_application) or [PWA](https://web.dev/progressive-web-apps/):
- Without user management and a database, we wouldn't need back-end at all. 
- SEO is not important.
- The first time render speed is not really important.
- We need to store and modify state often (for instance the timer represented as a running clock).

## Programming language & SPA framework:

- We want to write in a safe language.
   - It removes dynamically typed languages like Javascript from our options. And also languages that use potentially badly typed libraries like Typescript.
   
   - Also we don't want languages with many gotchas that allow us to write memory bugs like C++.

   - We don't want languages with too simple type system (no generics, `Result`, `Option/Maybe`, etc.) and with footguns like `null` - e.g. Go.

   - We can also filter languages by other rules like - _"Is it functional?"_, _"Does it support inheritance"_, _"Does it support only immutable and pure functions"_? However those rules are not as important as the ones mentioned above.

- Is language community big enough? I.e. we should be able to find usable frameworks, front-end libraries, tutorials, etc. And in the future also developers who will help us with the project.
  - [Elm](https://elm-lang.org/) + (no framework is needed) 
    - I would choose it if we need to support also older browsers that can't run WASM. Or perhaps if I have to mentor a junior front-end developer. 
    - I wouldn't use it if there is a big chance we would write a custom back-end (because we wouldn't be able to share types). Or when we need to write fast algorithms.
  - [Reason](https://reasonml.github.io/) + [ReasonReact](https://reasonml.github.io/reason-react/)
    - I would choose it if I like [React](https://reactjs.org/) but I want to use a sane language, supported by a large company. Or if I have to make an old big JS React project maintainable and there are JS/React-only developers in the team.
    - I wouldn't choose it if I don't want to use React or many compilers or [Node.js](https://nodejs.org/en/).
  - [C#](http://csharp.net/) + [Blazor](https://dotnet.microsoft.com/apps/aspnet/web-apps/blazor)
     - I would choose it if the company already use [.Net](https://dotnet.microsoft.com/) stack.
     - I wouldn't use it if the big bundle size is a problem. Or if I want a built-in state management.
  - [F#](https://fsharp.org/) + [Bolero](https://fsbolero.io/)
     - I would choose it if I want a more expressive language than C#.
     - I wouldn't choose it if I have to use only the most mainstream languages and officialy supported libraries.
  - [Purescript](https://www.purescript.org/) + [Halogen](https://purescript-halogen.github.io/purescript-halogen/)
    - I would choose it if I like Haskell, team members are experienced functional developers and Elm is too simple or opinionated for the project.
    - I wouldn't choose it if I don't want to use Node.js or [bower](https://bower.io/).
  - [Rust](https://www.rust-lang.org/) + [Seed](https://seed-rs.org/)
    - I would choose it if team members are Rust developers or have time to learn Rust. Or if I don't like writing untyped HTML and CSS.
    - I wouldn't choose it if I like JSX templates or unopinionated / modular frameworks.

- Other interesting front-end languages / frameworks: [Imba](https://www.imba.io/), [Mint](https://www.mint-lang.com/), [Yew](https://yew.rs/).

Well, I'm usre you aren't surprised that we'll use Rust + Seed to build our Time Tracker.

_Note:_ Please suggest changes in the list above when you find old, missing or incorrect information. I don't actively use the most of those frameworks.

## Styles

We don't have to use a custom design system or follow branding rules. Let's pick CSS framework compatible with Seed that covers the most website parts.

- There aren't many CSS-only frameworks. We can't and don't want to use any JS code because it'll probably break our website by direct DOM modifications.
- Also we don't want to use pure CSS because it's pain to write and maintain. [SASS](https://sass-lang.com/) or at least [LESS](http://lesscss.org/) would be better. And it should be quite popular so there are all needed components.

The only reasonable CSS framework seems to be [Bulma](https://bulma.io/).

## Version Control, CI pipeline and Hosting

The most proven and free combination is probably [GitHub](https://github.com/) + [GitHub Actions](https://github.com/features/actions) + [Netlify](https://www.netlify.com/).

Other possible combinations:
- [Azure Repos](https://azure.microsoft.com/en-us/services/devops/repos/) + [Azure Pipelines](https://azure.microsoft.com/en-us/services/devops/pipelines/)+ [Static Web Apps](https://azure.microsoft.com/en-us/services/app-service/static/)
- [GitLab](https://gitlab.com/) + [GitLab CI/CD](https://about.gitlab.com/stages-devops-lifecycle/continuous-integration/) + [GitLab Pages](https://docs.gitlab.com/ee/user/project/pages/)

Other free static site hostings are [GitHub Pages](https://pages.github.com/) and [Render](https://render.com/).

## Domain name

We don't need a custom domain name. Otherwise we would need to buy a domain like `time-tracker.com`. The choice of registrar heavily depends on the domain - e.g. I had to buy `kavik.cz` from the Czech registrar because the most known registrars often doesn't offer specific domains (e.g. `*.cz`).

You can often use the registrar's DNS servers or some hosting services like Netlify provide their own DNS servers and management.

Registrars: [Domain.com](https://www.domain.com/), [GoDaddy](https://uk.godaddy.com/), [Hover](https://www.hover.com/), [Dynadot](https://www.dynadot.com/), [Namecheap](https://www.namecheap.com/) and many [more](https://hostingfacts.com/domain-registrars/).
 
# Backend

Servers are often unnecessary complex and it's a pain to manage them. Also it's easy to kill a lot of time and create many security holes while writing custom back-end code. Let's make our back-end as simple as possible.

We would need to integrate 2 BE services - user management and managed database with a public API.

## User Management

- We need to choose IDaaS (Identity-as-a-service). The most known are probably [Auth0](https://auth0.com/) and [Okta](https://www.okta.com/). Both should support various registration/login flows and we should be able to meet the conditions of their free plans.

- Also each bigger cloud has own IDaaS - e.g. [Amazon Cognito](https://aws.amazon.com/cognito/) or [Azure Active Directory](https://azure.microsoft.com/en-us/services/active-directory/) or [Firebase Authentication](https://firebase.google.com/products/auth).

- And there is a chance that the chosen managed database offers its own identity provider.

## Database with a public API

When you look at our entities - User, Client, Project, .. (we've created a diagram in the one of the previous chapters) - and zoom out a bit, you'll notice that they form a tree.

### Database

- Our entity tree isn't very complex, so we can use all common database [types](https://dataguide.prisma.io/intro/comparing-database-types) like relational, document or graph. However the best fit for a tree of simple entities is probably a graph database. Read article [Why you should build your next app with a graph database](https://dgraph.io/blog/post/graphdb-for-your-next-app/) for detailed explanation.

- The most of the time we will be doing only simple [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) operations. The exception may be time entries aggregations to generate reports and show charts. 

- Our database should be fully managed - we don't want to think about the location, CPU count, memory consumption, number of connections, etc. And there should be a free plan ideally.

### Public API

- We can often choose between [REST](https://restfulapi.net/), [GraphQL](https://graphql.org/) or database-specific drivers that use [SQL](https://en.wikipedia.org/wiki/SQL) or DB-specific languages / protocols under the hood.

- Drivers are better for back-end integration, we don't need any special features and there are often available drivers for only the most mainstream languages. And we wouldn't be able to switch database or language later. So let's decide between REST and GraphQL.

- I would choose GraphQL because it's much better fit for our entity tree and (as you can guess from the name) it's the favorite query language for graph databases. Also we don't want to manually resolve problems like [N+1](https://restfulapi.net/rest-api-n-1-problem/).

- GraphQL-as-a-services:
  - [AWS AppSync](https://aws.amazon.com/appsync/)
  - [Fauna](https://fauna.com/)
  - [Slash GraphQL](https://dgraph.io/slash-graphql)
  - [GraphCMS](https://graphcms.com/)

We would like to use a free service and we don't want to fall into a huge ecosystem => no `AppSync` for us. We don't need the most of CMS features and there is a chance there we won't be able to write a more specialized custom queries => we won't connect to `GraphCMS`.

`Fauna` vs `Slash Graphql`. Fauna has a built-in authentication and a free plan. There are also some helpful articles how to integrate a custom identity provider to the stack with Fauna: e.g. [Using FaunaDB with an Identity Provider](https://www.felix-gehring.de/articles/2020/01/28/using-faunadb-with-an-identity-provider/) and [From Static Sites To End User JAMstack Apps With FaunaDB](https://www.smashingmagazine.com/2020/06/static-sites-jamstack-apps-faunadb/).
`Slash GraphQL` is (at the time of writing) private beta, however it looks promising. It's ready for 3rd-party identity providers (there is an `Auth0` integration tutorial in their docs) and it's based on a graph database.

Let's experiment a bit and choose `Slash GraphQL`. It would be nice to combine it with `Auth0`. There is a chance the API would be simpler thanks to the focusing only on the graph database and GraphQL. (I can't compare `Fauna` and `Slash GraphQL` properly now due to lack of experience with both services - don't hesitate to write me your opinions.)

---

So.. we've chosen:

- Frontend
   - Language: [Rust](https://www.rust-lang.org/)
   - SPA Framework: [Seed](https://seed-rs.org/)
   - CSS Framework: [Bulma](https://bulma.io/)
   - Version Control: [GitHub](https://github.com/)
   - CI/CD: [GitHub Actions](https://github.com/features/actions)
   - Hosting: [Netlify](https://www.netlify.com/)
   - Domain registrar: - (We'll se Netlify's sub-domain.)
- Backend
   - Identity Provider: [Auth0](https://auth0.com/)
   - GraphQL Backend: [Slash GraphQL](https://dgraph.io/slash-graphql)

--- 

## Future

Frontend
- Once Seed [Style and Hooks](https://seed-style-hooks.netlify.app/) are integrated, we expect there will be multiple Seed component libraries.

Backend 
- We want to make back-end integration as easy as possible. One of the option is also writing and managing open-source back-end layer focused on Seed.

- Please write us your current back-end integration challenges or server-related ideas or questions or how your planned/developed architecture looks like.
  - There is a [chat](https://discord.gg/JHHcHp5) channel `#app-architecture` or don't hesitate to contact me directly - chat or `martin@kavik.cz`

---

We've successfuly finished the most difficult tasks - writing specifications, requirements and choosing infrastructure components. Let's zoom in on front-end and describe a sitemap.
