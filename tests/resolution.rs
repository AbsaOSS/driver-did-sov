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

use std::thread;
use std::time::Duration;

use aries_vcx::common::ledger::{
    service_didsov::{DidSovServiceType, EndpointDidSov},
    transactions::write_endpoint,
};
use aries_vcx::utils::devsetup::SetupProfile;
use serde_json::Value;
use utils::send_request;

#[tokio::test]
async fn test_resolve_did() {
    SetupProfile::run(|init| async move {
        let did = format!("did:sov:{}", init.institution_did);
        let endpoint = EndpointDidSov::create()
            .set_service_endpoint("http://localhost:8080".parse().unwrap())
            .set_routing_keys(Some(vec!["key1".to_string(), "key2".to_string()]))
            .set_types(Some(vec![DidSovServiceType::Endpoint]));
        write_endpoint(&init.profile, &init.institution_did, &endpoint)
            .await
            .unwrap();
        thread::sleep(Duration::from_millis(50));
        let response = send_request(&did).await.unwrap();
        assert_eq!(response.status(), hyper::StatusCode::OK);
        let content_type = response.headers().get("content-type").unwrap();
        assert_eq!(content_type, "application/did+json");

        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let body_json: Value = serde_json::from_str(&body_str).unwrap();

        assert!(body_json.is_object());
        let did_document = body_json.get("didDocument").unwrap().as_object().unwrap();
        let id = did_document.get("id").unwrap().as_str().unwrap();
        assert_eq!(id, did);

        let service = did_document.get("service").unwrap().as_array().unwrap();
        assert_eq!(service.len(), 1);
        let service = service.get(0).unwrap().as_object().unwrap();
        let service_endpoint = service.get("serviceEndpoint").unwrap().as_str().unwrap();
        assert_eq!(service_endpoint, "http://localhost:8080/");

        let types = service.get("type").unwrap().as_array().unwrap();
        assert_eq!(types.len(), 1);
        let service_type = types.get(0).unwrap().as_str().unwrap();
        assert_eq!(service_type, "endpoint");
    })
    .await;
}
