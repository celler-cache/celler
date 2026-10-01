use anyhow::Result;
use clap::Parser;

use crate::cache::ServerName;
use crate::cli::Opts;
use crate::config::{Config, ServerConfig, ServerTokenConfig};

/// Log into an Attic server.
#[derive(Debug, Parser)]
pub struct Login {
    /// Name of the server.
    name: ServerName,

    /// Endpoint of the server.
    endpoint: String,

    /// Access token.
    #[clap(env = "CELLER_TOKEN")]
    token: Option<String>,

    /// Set the server as the default.
    #[clap(long)]
    set_default: bool,
}

pub async fn run(opts: Opts) -> Result<()> {
    let sub = opts.command.as_login().unwrap();
    let mut config = Config::load()?;

    if let Some(server) = config.servers.get_mut(&sub.name) {
        eprintln!("✍️ Overwriting server \"{}\"", sub.name.as_str());

        server.endpoint = sub.endpoint.to_owned();

        if let Some(token) = &sub.token {
            server.token = Some(ServerTokenConfig::Raw {
                token: token.clone(),
            });
        }
    } else {
        eprintln!("✍️ Configuring server \"{}\"", sub.name.as_str());

        config.servers.insert(
            sub.name.to_owned(),
            ServerConfig {
                endpoint: sub.endpoint.to_owned(),
                token: sub
                    .token
                    .to_owned()
                    .map(|token| ServerTokenConfig::Raw { token }),
            },
        );
    }

    if sub.set_default || config.servers.len() == 1 {
        config.default_server = Some(sub.name.to_owned());
    }

    config.save()
}
