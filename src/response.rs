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
    http::HeaderValue,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DIDJsonResponse(pub Value);

impl IntoResponse for DIDJsonResponse {
    fn into_response(self) -> Response {
        let mut res = Json(self.0).into_response();
        res.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/did+json"),
        );
        res
    }
}
