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

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use did_resolver_sov::{
    did_resolver::{
        did_parser::ParseError, traits::resolvable::resolution_error::DIDResolutionError,
    },
    error::DIDSovError,
};
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DidSovDriverError {
    #[error("Invalid DID: {0}")]
    ParseError(#[from] ParseError),
    #[error("Resolver error: {0}")]
    ResolveError(#[from] DIDSovError),
    #[error("Generic error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl IntoResponse for DidSovDriverError {
    fn into_response(self) -> Response {
        let handle_did_sov_error = |err: &DIDSovError| {
            let (status_code, description) = match err {
                DIDSovError::InvalidDID(_) => (
                    StatusCode::BAD_REQUEST,
                    DIDResolutionError::InvalidDid.to_string(),
                ),
                DIDSovError::NotFound(_) => (
                    StatusCode::NOT_FOUND,
                    DIDResolutionError::NotFound.to_string(),
                ),
                DIDSovError::MethodNotSupported(_) => (
                    StatusCode::NOT_IMPLEMENTED,
                    DIDResolutionError::MethodNotSupported.to_string(),
                ),
                DIDSovError::RepresentationNotSupported(_) => (
                    StatusCode::NOT_ACCEPTABLE,
                    DIDResolutionError::RepresentationNotSupported.to_string(),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    DIDResolutionError::InternalError.to_string(),
                ),
            };
            (
                status_code,
                json!({
                    "error": description,
                    "details": err.to_string()
                }),
            )
        };
        let handle_parse_error = |err: &ParseError| {
            error!("Parse error: {}", err);
            (
                StatusCode::BAD_REQUEST,
                json!({
                    "error": DIDResolutionError::InvalidDid.to_string(),
                    "details": err.to_string(),
                }),
            )
        };
        let handle_generic_error = |err: Box<dyn std::error::Error + Send + Sync>| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({
                    "error": DIDResolutionError::InternalError.to_string(),
                    "details": err.to_string(),
                }),
            )
        };
        let (status_code, body) = match self {
            DidSovDriverError::ParseError(err) => handle_parse_error(&err),
            DidSovDriverError::ResolveError(err) => handle_did_sov_error(&err),
            DidSovDriverError::Other(err) => {
                if let Some(err) = err.downcast_ref::<DIDSovError>() {
                    handle_did_sov_error(err)
                } else if let Some(err) = err.downcast_ref::<ParseError>() {
                    handle_parse_error(err)
                } else {
                    handle_generic_error(err)
                }
            }
        };
        IntoResponse::into_response((status_code, Json(body)))
    }
}
