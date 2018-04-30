//! [`inth_oauth2_slack`](https://crates.io/crates/inth_oauth2_slack) provides an implementation of [`inth-oauth2`](https://crates.io/crates/inth-oauth2)'s `Provider` trait for [Slack](https://slack.com/).
//!
//! # Examples
//!
//! ```rust
//! extern crate inth_oauth2 as oauth;
//! extern crate inth_oauth2_slack;
//!
//! let client = oauth::Client::new(
//!     inth_oauth2_slack::Slack,
//!     "<your client id>".into(),
//!     "<your client secret>".into(),
//!     Some("<your redirect url>".into())
//! );
//! 
//! // ...
//! ```

extern crate inth_oauth2 as oauth;
extern crate url;

#[macro_use]
extern crate lazy_static;

use oauth::{
    token::{Static, Bearer},
    provider::Provider,
};

use url::Url;

lazy_static! {
    static ref SLACK_AUTH_URI: Url = Url::parse("https://slack.com/oauth/authorize").unwrap();
    static ref SLACK_TOKEN_URI: Url = Url::parse("https://slack.com/api/oauth.access").unwrap();
}

/// The memberless struct that you'll use to authenticate with the Slack service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slack;

impl Provider for Slack {
    // Slack access tokens do not expire.
    type Lifetime = Static;
    type Token = Bearer<Self::Lifetime>;

    fn auth_uri(&self) -> &Url { &SLACK_AUTH_URI }
    fn token_uri(&self) -> &Url { &SLACK_TOKEN_URI }
}