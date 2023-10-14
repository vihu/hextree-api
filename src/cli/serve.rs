use crate::{server, settings::Settings};
use anyhow::Result;

/// Run the server
#[derive(Debug, clap::Args)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(&self, settings: &Settings) -> Result<()> {
        server::run(settings).await
    }
}
