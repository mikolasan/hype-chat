# Hype Chat


## About

I just started the development (May 2023). It's not my main side project, quite the opposite, this very random idea for practicing crazy stuff in between really important projects when it becomes boring even there.

Essentially this should become a normal IRC client and IRC server. But not that many people use IRC these days (at least me), so I'll focus on enhancing functionality of this simple, as it might seem, chat application.

Many hype features will be implemented first, even before core functionalities – this is the approach in this project.


## Roadmap

- End-to-end encryption with [Off-the-record messaging](https://en.wikipedia.org/wiki/Off-the-record_messaging).
- WebID authentication. DM communication stored in a Solid Pod
- Personalization: color schemes, user profile, content recommendation and filtering
- Video and audio calls
- Integration with third-party services: sharing files via google drive or dropbox, adding gifs to messages, displaying what is currently playing in your playlist
- The highest encryption level. User can manually select who can read their messages by sending decryption key, but then can revoke keys and change encryption and thus making all data hidden again.
- Runs on desktop and mobile
- Easy to develop new plugins: another integrations, another protocols, UI extensions
- Blogging platform
- Podcast platform
- Other features from [Friendica](https://friendi.ca/): integration with RSS, email, expiring content
- ...


## IRC protocol

- [Modern IRC Client Protocol](https://modern.ircdocs.horse/index.html)
- [(RFC 2812) Internet Relay Chat: Client Protocol](https://www.rfc-editor.org/rfc/rfc2812)


## Build and run

Server

```
cargo run --bin server
```

Client

```
cargo run --bin client
```
