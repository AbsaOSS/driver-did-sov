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

use utils::send_request;

#[tokio::test]
#[ignore = "Fails in CI"]
async fn test_resolver_performance() {
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

        let start = std::time::Instant::now();
        let num_requests = 100;
        for _ in 0..num_requests {
            let response = send_request(&did).await.unwrap();
            assert_eq!(response.status(), hyper::StatusCode::OK);
        }
        let elapsed = start.elapsed();
        println!("{num_requests} requests took: {elapsed:?}");
        assert!(elapsed < Duration::from_secs(10));
    })
    .await;
}
