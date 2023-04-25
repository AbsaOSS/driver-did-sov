# Universal Resolver Driver: did:sov

This is an alternative Universal Resolver driver for did:sov identifiers.

## Specifications

* [Decentralized Identifiers](https://w3c.github.io/did-core/)
* [DID Method Specification](https://sovrin-foundation.github.io/sovrin/spec/did-method-spec-template.html)

## Example DIDs

```
did:sov:WRfXPg8dantKVubE3HX8pw
```

## Build and Run (Docker Compose)

```
./scripts/build_and_run.sh
./scripts/resolve.sh did:sov:WRfXPg8dantKVubE3HX8pw
```

## Driver Environment Variables

The driver recognizes the following environment variables:

### `APP_CONFIG`

 * Designates a predefined configuration. Possible values are: localhost, staging, main.
 * Default value: (empty string)

### `WALLET::KEY`

 * The key to use for the Indy wallet.
 * Default value: (empty string)

### `WALLET::NAME`

 * Name of the Indy wallet.
 * Default value: (empty string)

### `WALLET::KDF`

 * Key derivation function for the Indy wallet.
 * Default value: (empty string)

### `POOL::NAME`

 * Local name of the pool to open.
 * Default value: (empty string)

### `POOL::NETWORK`

 * Name of the Indy network to connect to. Possible values are:
 localhost, staging, main.
 * Default value: (empty string)

### `APPLICATION::PORT`

 * Port for the server to listen to incoming requests on.
 * Default value: (empty string)

### `APPLICATION::LOG_LEVEL`

 * Log level for the application.
 * Default value: (empty string)

## Driver Metadata

The driver returns the following metadata in addition to a DID document:

* `didResolutionMetadata`: DID resolution metadata as defined
  [here](https://www.w3.org/TR/did-core/#dfn-didresolutionmetadata).
* `didDocumentMetadata`: DID document metadata as defined [here](https://www.w3.org/TR/did-core/#dfn-diddocumentmetadata).

---
    Copyright 2023 ABSA Group Limited
    
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at
    
        http://www.apache.org/licenses/LICENSE-2.0
    
    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
