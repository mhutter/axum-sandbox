# Axum Experiments


## Sessions

Source: [session.rs](src/experiments/session.rs)

Goal:

- use & modify session data
- data is stored in a cookie (usually an encrypted one, but we use a plan text one here for testing purposes)
- more ergonomic than `CookieJar` that needs to be included in every response manually
- type-safety
