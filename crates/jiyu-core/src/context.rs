use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use bon::Builder;
use non_empty_str::{NonEmptyCowStr, NonEmptyStr, const_non_empty_str};

#[cfg(feature = "ownership")]
use ownership::IntoOwned;
use tracing::info;

use crate::ensure::{Error as EnsureError, ensure};

/// Represents context paths.
pub type ContextPath<'c> = Cow<'c, Path>;

/// Represents context names.
pub type ContextName<'c> = NonEmptyCowStr<'c>;

pub type StaticPath = &'static Path;
pub type StaticName = &'static NonEmptyStr;

pub const DEFAULT_JSON: bool = false;
pub const DEFAULT_ENSURE: bool = true;

pub const DEFAULT_XRAY: &str = "xray";
pub const DEFAULT_SING_BOX: &str = "sing-box";
pub const DEFAULT_STATE: &str = "/var/lib";

pub const DEFAULT_DIRECTORY: &str = "jiyu";
pub const DEFAULT_CONFIG: &str = "config";
pub const DEFAULT_CHECK: &str = "check";

pub const DEFAULT_NON_EMPTY_XRAY: &NonEmptyStr = const_non_empty_str!(DEFAULT_XRAY);
pub const DEFAULT_NON_EMPTY_SING_BOX: &NonEmptyStr = const_non_empty_str!(DEFAULT_SING_BOX);
pub const DEFAULT_NON_EMPTY_STATE: &NonEmptyStr = const_non_empty_str!(DEFAULT_STATE);

pub const DEFAULT_NON_EMPTY_DIRECTORY: &NonEmptyStr = const_non_empty_str!(DEFAULT_DIRECTORY);
pub const DEFAULT_NON_EMPTY_CONFIG: &NonEmptyStr = const_non_empty_str!(DEFAULT_CONFIG);
pub const DEFAULT_NON_EMPTY_CHECK: &NonEmptyStr = const_non_empty_str!(DEFAULT_CHECK);

pub const fn default_json() -> bool {
    DEFAULT_JSON
}

pub const fn default_ensure() -> bool {
    DEFAULT_ENSURE
}

pub fn default_xray() -> StaticPath {
    Path::new(DEFAULT_XRAY)
}

pub fn default_sing_box() -> StaticPath {
    Path::new(DEFAULT_SING_BOX)
}

pub fn default_state() -> StaticPath {
    Path::new(DEFAULT_STATE)
}

pub const fn default_directory() -> StaticName {
    DEFAULT_NON_EMPTY_DIRECTORY
}

pub const fn default_config() -> StaticName {
    DEFAULT_NON_EMPTY_CONFIG
}

pub const fn default_check() -> StaticName {
    DEFAULT_NON_EMPTY_CHECK
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
#[cfg_attr(feature = "ownership", derive(IntoOwned))]
pub struct Context<'c> {
    #[builder(into, default = default_json())]
    pub json: bool,

    #[builder(into, default = default_ensure())]
    pub ensure: bool,

    #[builder(into, default = default_xray())]
    pub xray: ContextPath<'c>,

    #[builder(into, default = default_sing_box())]
    pub sing_box: ContextPath<'c>,

    #[builder(into, default = default_state())]
    pub state: ContextPath<'c>,

    #[builder(into, default = default_directory())]
    pub directory: ContextName<'c>,

    #[builder(into, default = default_config())]
    pub config: ContextName<'c>,

    #[builder(into, default = default_check())]
    pub check: ContextName<'c>,
}

pub type OwnedContext = Context<'static>;

impl Default for Context<'_> {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Builder)]
pub struct Ensure {
    directory: bool,
    config: bool,
    check: bool,
}

impl Context<'_> {
    pub const fn json(&self) -> bool {
        self.json
    }

    pub const fn ensure(&self) -> bool {
        self.ensure
    }

    pub fn xray(&self) -> &Path {
        self.xray.as_ref()
    }

    pub fn sing_box(&self) -> &Path {
        self.sing_box.as_ref()
    }

    pub fn state(&self) -> &Path {
        self.state.as_ref()
    }

    pub fn directory(&self) -> &str {
        self.directory.as_ref()
    }

    pub fn config(&self) -> &str {
        self.config.as_ref()
    }

    pub fn check(&self) -> &str {
        self.check.as_ref()
    }

    pub fn directory_path(&self) -> PathBuf {
        self.state().join(self.directory())
    }

    pub fn config_path(&self) -> PathBuf {
        self.directory_path().join(self.config())
    }

    pub fn check_path(&self) -> PathBuf {
        self.directory_path().join(self.check())
    }

    pub async fn prepare(&self) -> Result<Ensure, EnsureError> {
        let create = self.ensure();

        let directory_path = self.directory_path();

        info!(
            "ensuring `directory` at `{display}`",
            display = directory_path.display()
        );

        let directory = ensure()
            .directory(directory_path)
            .create(create)
            .call()
            .await?;

        let config_path = self.config_path();

        info!(
            "ensuring `config` at `{display}`",
            display = config_path.display()
        );

        let config = ensure()
            .directory(config_path)
            .create(create)
            .call()
            .await?;

        let check_path = self.check_path();

        info!(
            "ensuring `check` at `{display}`",
            display = check_path.display()
        );

        let check = ensure().directory(check_path).create(create).call().await?;

        let output = Ensure::builder()
            .directory(directory)
            .config(config)
            .check(check)
            .build();

        Ok(output)
    }
}
