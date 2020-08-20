# Technical Requirements

_"Simply put, the difference is that non-functional (technical) requirements describe how the system works, while functional requirements describe what the system should do."_ 
   - from the article [Functional vs Non Functional Requirements](https://reqtest.com/requirements-blog/functional-vs-non-functional-requirements/)

---

So let's define our requirements. The list below may contain also some functional requirements that affects architecture or design. 

---

Time Tracker users don't want to lose their data and the app will be used as a live Seed example, so:

1. The app should be available worldwide without limitations and ideally with the same performance and latency.

1. The hosting should be free or very cheap.

1. The app content (labels, images, etc.; not user data) may be hard-coded - English would be the only language. (See the example [i18n](https://github.com/seed-rs/seed/tree/6238190b5441b283df4fdb49078cccf420b512a4/examples/i18n) when you need to support more languages.)

1. The app should be reliable to not disrupt user work - especially while the user is tracking time.

1. User data have to be stored securely and reliably.

1. SEO is not important - we don't need [prerendering](https://www.netlify.com/blog/2016/11/22/prerendering-explained/) or [server-side rendering](https://blog.jakoblind.no/getting-started-react-ssr/).
   - If you want to prerender your app as a build step, you can use Seed [Webpack quickstart](https://github.com/seed-rs/seed-quickstart-webpack) (or [Seeder](https://github.com/MartinKavik/seeder) in the future).
   - Seed SSR is planned, but it's not a priority now.

1. The app don't have to respect all [accessibility](https://developer.mozilla.org/en-US/docs/Learn/Accessibility/What_is_accessibility) best practices - we assume that the most of users will be developers without disabilities and accessibility itself is out-of-scope of this tutorial.

1. We don't have to use any existing backend.

1. We don't have to follow any branding guidelines or design systems.

1. The maintenance should be as easy as possible. We don't want to care about servers, hosting, regular updates, etc.

1. The responsive GUI would be nice, but it's not the top priority.

1. We don't want to integrate a [cookie banner](https://secureprivacy.ai/why-you-need-a-cookie-banner-on-your-website/). The app should use only strictly necessary [cookies](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies).

1. We expect that users won't create many entities and that the traffic amount won't be very high. However scalability would be nice.

1. We don't need any realtime communication. It means we don't have to integrate something like [WebSockets](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API) or [Server-send events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events).

1. We don't have to support old or "exotic" browsers.

1. We don't need to support "offline mode" (The app is usable even without internet connection). However it would be a reasonable future requirement for the time tracking app - then we should look also at [Progressive Web Apps](https://web.dev/progressive-web-apps/) and their builders like [Woz](https://github.com/alexkehayias/woz).

1. We can use external managed services (identity providers, mail senders, databases, ...)

1. We don't want to register a custom domain. Something like `time-tracker.static_site_hosting.com` would be enough.

---

Let's move to architecture before we forget all specifications and requirements...











