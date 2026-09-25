/// The `jiyu` version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The `jiyu/version` user agent to use for requests.
pub const USER_AGENT: &str = concat!("jiyu", "/", env!("CARGO_PKG_VERSION"));
