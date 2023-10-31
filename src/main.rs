use anyhow::{bail, Result};
use clap::Parser;
use hextree_api::{cli::serve, error::AppError, settings::Settings};
use std::path::PathBuf;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Debug, clap::Subcommand)]
pub enum Cmd {
    Serve(serve::Cmd),
}

impl Cmd {
    pub async fn run(self, settings: Settings) -> Result<(), AppError> {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&settings.log))
            .with(tracing_subscriber::fmt::layer())
            .init();

        match self {
            Self::Serve(cmd) => Ok(cmd.run(&settings).await?),
        }
    }
}

#[derive(Debug, clap::Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "HexTree API")]
pub struct Cli {
    #[clap(short = 'c')]
    config: Option<PathBuf>,

    #[clap(subcommand)]
    cmd: Cmd,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let settings = Settings::new(self.config)?;

        match self.cmd.run(settings).await {
            Ok(_) => Ok(()),
            Err(err) => {
                bail!("Error: {:?}", err)
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await
}
