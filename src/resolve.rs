use axum::extract::State;
use axum::{extract::Path, Extension};
use did_resolver_sov::did_resolver::traits::resolvable::resolution_output::DIDResolutionOutput;
use did_resolver_sov::did_resolver::traits::resolvable::DIDResolvable;
use did_resolver_sov::did_resolver::{
    did_parser::ParsedDID, traits::resolvable::resolution_options::DIDResolutionOptions,
};
use did_resolver_sov::resolution::DIDSovResolver;
use lru::LruCache;
use serde_json::json;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use crate::error::DidSovDriverError;
use crate::response::DIDJsonResponse;

async fn is_cached(
    cache: &Arc<Mutex<LruCache<String, (Instant, DIDJsonResponse)>>>,
    did: &str,
) -> Option<DIDJsonResponse> {
    let cache_ttl = Duration::from_secs(60);

    let mut cache_lock = cache.lock().await;
    match cache_lock.get_mut(did) {
        Some((instant, response)) if instant.elapsed() < cache_ttl => {
            debug!("Cache hit for DID {}", did);
            Some(response.clone())
        }
        Some((_, _)) => {
            debug!("Cache expired for DID {}", did);
            cache_lock.pop(did);
            None
        }
        None => None,
    }
}

async fn build_did_json_response(resolution_output: DIDResolutionOutput) -> DIDJsonResponse {
    DIDJsonResponse(json!({
        "didDocument": resolution_output.did_document(),
        "didResolutionMetadata": resolution_output.did_resolution_metadata(),
        "didDocumentMetadata": resolution_output.did_document_metadata(),
    }))
}

async fn resolve_did_without_cache(
    did: String,
    resolver: &Arc<DIDSovResolver>,
) -> Result<DIDJsonResponse, DidSovDriverError> {
    let parsed_did = ParsedDID::parse(did)?;
    let resolution_output = resolver
        .resolve(&parsed_did, &DIDResolutionOptions::default())
        .await?;

    Ok(build_did_json_response(resolution_output).await)
}

async fn handle_cache(
    cache: &Arc<Mutex<LruCache<String, (Instant, DIDJsonResponse)>>>,
    did: String,
    response: DIDJsonResponse,
) {
    cache.lock().await.put(did, (Instant::now(), response));
}

pub async fn resolve_did(
    Path(did): Path<String>,
    Extension(resolver): Extension<Arc<DIDSovResolver>>,
    State(cache): State<Arc<Mutex<LruCache<String, (Instant, DIDJsonResponse)>>>>,
) -> Result<DIDJsonResponse, DidSovDriverError> {
    if let Some(response) = is_cached(&cache, &did).await {
        return Ok(response);
    }

    let response = resolve_did_without_cache(did.clone(), &resolver).await?;

    handle_cache(&cache, did.clone(), response.clone()).await;

    Ok(response)
}
