use std::path::PathBuf;

use crate::{
    util::{node_rpc, random_name},
    vault::default_vault_name,
    CommandGlobalOpts,
};
use anyhow::anyhow;

use clap::Args;
use ockam::Context;
use ockam_api::cli_state::CredentialConfig;
use ockam_identity::IdentityIdentifier;

#[derive(Clone, Debug, Args)]
pub struct StoreCommand {
    #[arg(hide_default_value = true, default_value_t = random_name())]
    pub credential_name: String,

    #[arg(long = "issuer")]
    pub issuer: IdentityIdentifier,

    #[arg(long = "subject")]
    pub subject: IdentityIdentifier,

    #[arg(group = "credential_value", value_name = "CREDENTIAL_STRING", long)]
    pub credential: Option<String>,

    #[arg(group = "credential_value", value_name = "CREDENTIAL_FILE", long)]
    pub credential_path: Option<PathBuf>,

    #[arg(default_value_t = default_vault_name())]
    pub vault: String,
}

impl StoreCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        node_rpc(run_impl, (opts, self));
    }
}

async fn run_impl(
    _ctx: Context,
    (opts, cmd): (CommandGlobalOpts, StoreCommand),
) -> crate::Result<()> {
    let cred_as_str = match (cmd.credential, cmd.credential_path) {
        (_, Some(credential_path)) => tokio::fs::read_to_string(credential_path).await?,
        (Some(credential), _) => credential,
        _ => return Err(anyhow!("Credential or Credential Path argument must be provided").into()),
    };

    // store
    opts.state
        .credentials
        .create(
            &cmd.credential_name,
            CredentialConfig::new(cmd.issuer.to_string(), cred_as_str)?,
        )
        .await?;

    println!("Credential {} stored", &cmd.credential_name);

    Ok(())
}