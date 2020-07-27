
1.
```
near login
```

2. 
```
near deploy --wasmFile output/DID_NEAR_rust_optimized.wasm --accountId test_did.testnet
```

3. 
```
near call test_did.testnet reg_did_using_account --accountId test_did.testnet
```

4. 
```
near view test_did.testnet get_document '{"did":"did:near:test_did.testnet"}' --accountId test_did.testnet
```