/// Uses `.env` if exist
/// Docs: [`clap`](https://github.com/clap-rs/clap/)

use clap;


#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,
}