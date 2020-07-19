# Sitemap

Let's design app's pages. 

Included pictures below are basically ugly mockups with some missing component states where some parts are just copy & pasted from Bulma's docs. However it should be enough to guide us during implementation.

_Notes:_ 
- Teaching UI/UX theory is out of scope of this tutorial. However we'll try to design & develop something pretty and usable with a bit of luck.
- If you want to learn something about UX/UI quickly, I recommend to look at [Refactoring UI](https://refactoringui.com/) and [Nick Kolenda's UX list](https://www.nickkolenda.com/user-experience/). Or [start.uxdesign.cc](https://start.uxdesign.cc/) is a good source of inspiration.
- I've "designed" those mockups in [Figma](https://www.figma.com/). I can recommend to try also [Adobe XD](https://www.adobe.com/products/xd.html) or [Affinity Designer](https://affinity.serif.com/en-gb/designer/).

---

## Home

- URL: `/`
- It will be used only as a crossroad. There could be simple button "Go to Time Tracker" as a CTA / shortcut.
- No item in the main menu is active.
- If the user is logged in, show buttons "Sign up" and "Log in". 
- If the user isn't logged in, show buttons "[user_name]" and "Log out". The first button navigates to the page `Settings`.
- `Bulma` logo will be replaced.
- `seed-rs.org` should be a link.
- Themes / colors may be changed later.

![Design Home](/static/images/design_home.png)

---

## Login / Registration

- It will be handled by our identity provider.
- It should look like this: ![Auth0 Universal Login](/static/images/auth0_universal_login.png)
- Doc: [auth0.com/docs/universal-login](https://auth0.com/docs/universal-login)
- (image original [location](https://auth0.com/blog/introducing-the-new-auth0-universal-login-experience/))

---

## User Management

- It will be also handled by our identity provider. ![Auth0 User Management](/static/images/auth0_user_management.png)

- Docs: [auth0.com/docs/users/guides/manage-users-using-the-dashboard](https://auth0.com/docs/users/guides/manage-users-using-the-dashboard)

- _Note:_ I didn't find a way how to delete user's data automatically when the user is deleted through `Auth0` user dashboard (by a trigger, rule, webhook, ..). You'll probably need to delete it manually. (Am I wrong? Write me!)

---

## Clients & Projects

- URL: `/clients_and_projects`
- User can add, remove and rename his clients and projects.
- All removing operations should be confirmed by a simple modal dialog box. We'll use ugly browser native one for the sake of simplicity.
- Client and project names are in-place editable.
- A [trash icon](https://fontawesome.com/icons/trash-alt?style=solid) would be probably more appropriate than a standard Bulma's _delete button_ with the "X" icon.

![Design Clients & Projects](/static/images/design_clients_and_projects.png)

- _Note:_ SPA frameworks usually don't like font libraries very much because those libraries often changes DOM elements by themselves. We'll leverage [custom elements](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_custom_elements) to eliminate this problem while we will be integrating [Font Awesome](https://fontawesome.com/).

---

## Time Tracker

- URL: `/time_tracker`
- User can create new Time Entries through "Start/Stop" button.
- When the Time Entry is active, its background is highlighted and its "end" time is updated each second to match the current time. 

![Design Time Tracker](/static/images/design_time_tracker.png)

---

## Time Blocks

- URL: `/time_blocks`
- User can add and modify Time Blocks.
- All statistics are automatically recomputed and read-only.

![Design Time Blocks](/static/images/design_time_blocks.png)

---

## Settings

- URL: `/settings`
- It'll communicate only with our identity provider. (An exception could be deleting an account.)
- We don't need it at all if our IDP offers its own user settings.

![Design Settings](/static/images/design_settings.png)

---

Architecture and UI are designed, the next step is the front-end `Model`.
