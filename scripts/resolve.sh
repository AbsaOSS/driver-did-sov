DID=$1
curl -H "Accept: application/did+json" http://127.0.0.1:4000/1.0/identifiers/${DID}
