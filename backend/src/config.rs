use std::path::PathBuf;

use clap::Args;

#[derive(clap::Parser)]
pub struct Config {
    #[command(flatten)]
    pub db_config: DBConfig,

    #[clap(long, short = 'p')]
    /// Dynamically serve the web frontend from this path instead of the bundled frontend
    pub frontend_path: Option<PathBuf>,
    #[clap(long)]
    /// Do not serve a frontend webpage
    pub no_ui: bool,
}

#[derive(Args)]
#[group(required = true)]
pub struct DBConfig {
    #[clap(long, env)]
    pub database_url: Option<String>,
    #[clap(long, action)]
    /// Run without a database, all API calls will return a default response, intended for testing purposes
    pub no_db: bool,
}
