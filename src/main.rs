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

#[macro_use]
extern crate log;

mod config;
mod error;
mod init;
mod resolve;
mod response;

use anyhow::Context;
use axum::Server;
use axum::{routing::get, Extension, Router};
use lru::LruCache;
use response::DIDJsonResponse;
use std::num::NonZeroUsize;
use std::time::Instant;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::init::initialize_resolver_from_config;
use resolve::resolve_did;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::new()?;

    tracing_subscriber::fmt()
        .with_max_level(config.application.log_level.0)
        .init();

    let resolver = initialize_resolver_from_config(&config).await?;
    let cache =
        LruCache::<String, (Instant, DIDJsonResponse)>::new(NonZeroUsize::new(100).unwrap());
    let app = Router::new()
        .route("/1.0/identifiers/:did", get(resolve_did))
        .layer(Extension(Arc::new(resolver)))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(Mutex::new(cache)));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.application.port));
    info!("Server listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Server failed")
}
