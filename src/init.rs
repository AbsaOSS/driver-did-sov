/*
 * Copyright 2023 ABSA Group Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{num::NonZeroUsize, sync::Arc};

use crate::config::Config;
use aries_vcx::{
    aries_vcx_core::{
        indy::{
            ledger::pool::{create_pool_ledger_config, open_pool_ledger, PoolConfigBuilder},
            wallet::{create_wallet_with_master_secret, open_wallet, WalletConfigBuilder},
        },
        PoolHandle, WalletHandle,
    },
    core::profile::{profile::Profile, vdrtools_profile::VdrtoolsProfile},
};
use did_resolver_sov::resolution::DIDSovResolver;

fn prepare_genesis_path(config: &Config) -> Result<String, anyhow::Error> {
    let base_path = std::env::current_dir()?;
    let genesis_directory = base_path.join("genesis");
    Ok(genesis_directory
        .join(format!("{}.txn", config.pool.network))
        .display()
        .to_string())
}

async fn create_wallet(config: &Config) -> Result<WalletHandle, anyhow::Error> {
    let config_wallet = WalletConfigBuilder::default()
        .wallet_name(config.wallet.name.as_str())
        .wallet_key(config.wallet.key.as_str())
        .wallet_key_derivation(config.wallet.kdf.as_str())
        .build()?;
    create_wallet_with_master_secret(&config_wallet).await?;
    open_wallet(&config_wallet).await.map_err(|err| err.into())
}

async fn open_pool(config: &Config, genesis_path: &str) -> Result<PoolHandle, anyhow::Error> {
    let pool_config = PoolConfigBuilder::default()
        .genesis_path(genesis_path)
        .build()?;
    create_pool_ledger_config(&config.pool.name, genesis_path)?;
    Ok(open_pool_ledger(&config.pool.name, Some(pool_config)).await?)
}

pub async fn initialize_resolver_from_config(
    config: &Config,
) -> Result<DIDSovResolver, anyhow::Error> {
    let genesis_path = prepare_genesis_path(config)?;

    let wallet_handle = create_wallet(config).await?;
    let pool_handle = open_pool(config, &genesis_path).await?;

    let profile: Arc<dyn Profile> = Arc::new(VdrtoolsProfile::new(wallet_handle, pool_handle));

    Ok(DIDSovResolver::new(
        profile.inject_ledger(),
        NonZeroUsize::new(10).unwrap(),
    ))
}
