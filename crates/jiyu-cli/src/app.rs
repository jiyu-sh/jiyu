use std::{io::Error as IoError, path::PathBuf};

pub use anyhow::Result;

use clap::{Args, Parser, Subcommand};
use non_empty_str::NonEmptyString;
use thiserror::Error;

use jiyu_core::{
    context::{
        Context, DEFAULT_CHECK, DEFAULT_CONFIG, DEFAULT_DIRECTORY, DEFAULT_SING_BOX, DEFAULT_STATE,
        DEFAULT_XRAY, OwnedContext,
    },
    ensure::Error as EnsureError,
};
use tokio::runtime::{Builder, Runtime};
use tracing::info;

use crate::tracing::{Error as InitError, init};

#[derive(Debug, Args)]
pub struct Globals {
    #[arg(
        global = true,
        short = 'J',
        long,
        help = "Emit json instead of human-readable output",
        action,
        env = "JIYU_JSON"
    )]
    pub json: bool,

    #[arg(
        global = true,
        short = 'e',
        long,
        help = "Create the directories if they do not exist",
        action,
        env = "JIYU_ENSURE"
    )]
    pub ensure: bool,

    #[arg(
        global = true,
        short = 'x',
        long,
        help = "Use this xray binary",
        default_value = DEFAULT_XRAY,
        env = "JIYU_XRAY"
    )]
    pub xray: PathBuf,

    #[arg(
        global = true,
        short = 's',
        long,
        help = "Use this sing-box binary",
        default_value = DEFAULT_SING_BOX,
        env = "JIYU_SING_BOX"
    )]
    pub sing_box: PathBuf,

    #[arg(
        global = true,
        short = 'S',
        long,
        help = "Use this state directory",
        default_value = DEFAULT_STATE,
        env = "JIYU_STATE"
    )]
    pub state: PathBuf,

    #[arg(
        global = true,
        short = 'd',
        long,
        help = "Use this directory",
        default_value = DEFAULT_DIRECTORY,
        env = "JIYU_DIRECTORY"
    )]
    pub directory: NonEmptyString,

    #[arg(
        global = true,
        short = 'c',
        long,
        help = "Use this config directory",
        default_value = DEFAULT_CONFIG,
        env = "JIYU_CONFIG"
    )]
    pub config: NonEmptyString,

    #[arg(
        global = true,
        short = 'C',
        long,
        help = "Use this check directory",
        default_value = DEFAULT_CHECK,
        env = "JIYU_CHECK"
    )]
    pub check: NonEmptyString,
}

impl Globals {
    pub fn into_context(self) -> OwnedContext {
        OwnedContext::builder()
            .json(self.json)
            .ensure(self.ensure)
            .xray(self.xray)
            .sing_box(self.sing_box)
            .state(self.state)
            .directory(self.directory)
            .config(self.config)
            .check(self.check)
            .build()
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct App {
    /// The global options to use.
    #[command(flatten)]
    pub globals: Globals,

    /// The command to run.
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Error)]
#[error("failed to build runtime: {error}")]
pub struct BuildError {
    #[from]
    error: IoError,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Init(#[from] InitError),
    Build(#[from] BuildError),
    Run(#[from] RunError),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RunError {
    Ensure(#[from] EnsureError),
    Command(#[from] CommandError),
}

impl App {
    pub fn build() -> Result<Runtime, BuildError> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;

        Ok(runtime)
    }

    pub fn split(self) -> (Globals, Command) {
        (self.globals, self.command)
    }

    pub fn build_run(self) -> Result<(), Error> {
        let (globals, command) = self.split();

        let context = globals.into_context();

        init()?;

        info!("building runtime");

        let runtime = Self::build()?;

        let future = Self::run(context, command);

        runtime.block_on(future)?;

        Ok(())
    }

    pub async fn run(context: Context<'_>, command: Command) -> Result<(), RunError> {
        context.prepare().await?;

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Ping(PingCommand),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum CommandError {
    Ping(#[from] PingError),
}

#[derive(Debug, Args)]
pub struct PingCommand;

#[derive(Debug, Error)]
pub enum PingError {}

impl PingCommand {
    pub async fn run(self) -> Result<(), PingError> {
        Ok(())
    }
}

pub fn run() -> Result<()> {
    App::parse().build_run()?;

    Ok(())
}
