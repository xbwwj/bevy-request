# Review of Bevy HTTP client implementations

> [!NOTE]
>
> This review is done by the authors of [bevy-request](#), which is another bevy
> http client library. Information provided here may be biased.

In this file we do an extensive search and review for bevy http client
implementation. The scope includes projects on crates.io, lib.rs and GitHub.

- Search keywords: `#bevy`, `#http`
- Scope:
  - [x] lib.rs
  - [x] crates.io
  - [x] github.com
- Last updated at 2026-01-05Z

## Overview

| Project                             | Status      | Features                      |
| ----------------------------------- | ----------- | ----------------------------- |
| [bevy_ehttp][be]                    | ✅ Active   | `ehttp`, `EntityEvent`        |
| [bevy_flurx_api][bfa]               | ✅ Active   | tied to `bevy_flurx`          |
| [bevy_http_client][bhc]             | ✅ Active   | `ehttp`, `EntityEvent`        |
| [bevy_mod_reqwest][bmr]             | ✅ Active   | `reqwest`, `EntityEvent`      |
| [bevy_stardust][bs]                 | ❌ Outdated | custom channel                |
| [pecs_http][ph]                     | ❌ Outdated | tied to `pecs`                |
| [bevy_http][bh]                     | ❌ Outdated | `surf`, `AssetServer`         |
| [bevy_defer_http][bdh]              | ❌ Outdated | `hyper`, tied to `bevy_defer` |
| [StrikeForceZero/bevy_http][sfz/bh] | ❌ Outdated |                               |

Specialized client:

| Project                 | Status    | Features                     |
| ----------------------- | --------- | ---------------------------- |
| [bevy-discord][bd]      | ✅ Active | Discord, `Message`           |
| [bevy_blob_loader][bbl] | ✅ Active | for browser `blob:` url only |
| [bevy_web_asset][bwa]   | 🔀 Merged | Asset only, `ureq`           |

[bwa]: https://github.com/johanhelsing/bevy_web_asset
[be]: https://github.com/leinnan/bevy_ehttp
[bmr]: https://github.com/totalkrill/bevy_mod_reqwest
[bs]: https://codeberg.org/veritius/bevy_stardust
[bhc]: https://github.com/foxzool/bevy_http_client
[ph]: https://github.com/jkb0o/pecs
[bfa]: https://github.com/not-elm/bevy_webview_projects
[bbl]: https://github.com/kayhhh/bevy_blob_loader
[bh]: https://github.com/lizelive/bevy_http
[bdh]: https://github.com/mintlu8/bevy_defer
[bd]: https://github.com/AS1100K/bevy-discord
[sfz/bh]: https://github.com/StrikeForceZero/bevy_http

## Evaluate

In this section, we will evaluate these libraries from the following aspects:

- HTTP backend: `ureq` vs `reqwest` vs others
- Callback mechanism: `EntityEvent` vs `Message{,Reader}`
- ECS integration: sadly none

### HTTP backend

`ureq` and `reqwest` are the two most popular http client in rust ecosystem.
Both of them are okay, while the latter one may increase binary size due to its
async stack.

There is also other http backends. `ehttp` is a ergonomic wrapper around `ureq`
at the cost of losing many advanced features like proxies. `surf` is now
deprecated and out of consideration. `nyquest` is another noteworthy http client
featuring platform native http stack.

Using `ureq` or `reqwest` as backend does not necessarily ensure the client is
feature complete. For example, `bevy_mod_reqwest` use a `Client` resource as
entry point, while both `ureq` and `reqwest` need `Client`/`Agent` customization
to use proxy.

### Callback mechanism

`EntityEvent` feels much more natural than `Message`. There is a one to one
relation between request and response. The `EntityEvent` can convey request
information through entity, which `Message` squeeze all response together.

Though `bevy_mod_reqwest` also employs the `EntityEvent` approach, this library
use it in a surprising way, with custom `on_response`, `on_json_response`,
`on_error` handler rather standard `observe`.

### ECS integration

Sophisticated Bevy users may expect HTTP request to be natively represented as
composable Bevy components. For example,

```rust
commands
    .spawn((GET, "https://example.com", headers, proxy, extra_options))
    .observe(|response: On<Response>| {
        println!("{}", response.status());
    })
    .ovserve(|error: On<RequestError>| {
        eprintln!("{error:?}");
    });
```

However, all current implementations simply behave as a glue between bevy system
and http client.
