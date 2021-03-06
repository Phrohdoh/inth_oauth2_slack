# `inth_oauth2_slack`

Provides an implementation of [`inth-oauth2`](https://crates.io/crates/inth-oauth2)'s `Provider` trait for [Slack](https://slack.com/).

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![docs.rs](https://docs.rs/inth_oauth2_slack/badge.svg)](https://docs.rs/inth_oauth2_slack)
[![crates.io](https://meritbadge.herokuapp.com/inth_oauth2_slack)](https://crates.io/crates/inth_oauth2_slack)

## Examples

```rust
extern crate inth_oauth2 as oauth;
extern crate inth_oauth2_slack;

let client = oauth::Client::new(
    inth_oauth2_slack::Slack,
    "<your client id>".into(),
    "<your client secret>".into(),
    Some("<your redirect url>".into())
);

// ...
```

## License

See `LICENSE`.