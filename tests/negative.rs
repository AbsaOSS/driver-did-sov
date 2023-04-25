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

mod utils;

use utils::send_request;

#[tokio::test]
async fn test_resolve_non_existent_did() {
    let non_existent_did = "did:sov:KxDPhdCQ2YhKuVzKnAJiSU";

    let response = send_request(non_existent_did).await.unwrap();

    assert_eq!(response.status(), hyper::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_resolve_malformed_did_uri() {
    let malformed_did_uri = "did:sov:malformedDID!";

    let response = send_request(malformed_did_uri).await.unwrap();

    assert_eq!(response.status(), hyper::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_resolve_unsupported_did_method() {
    let unsupported_did_method = "did:unsupported:KxDPhdCQ2YhKuVzKnAJiSU";

    let response = send_request(unsupported_did_method).await.unwrap();

    assert_eq!(response.status(), hyper::StatusCode::NOT_IMPLEMENTED);
}
