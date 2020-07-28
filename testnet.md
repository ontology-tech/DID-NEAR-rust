
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
near call test_did.testnet add_controller '{"controller":"did:near:test_did.testnet"}' --accountId test_did.testnet
```

5. 
```
near call test_did.testnet add_key '{"pk":[0,1], "controller":"did:near:test_did.testnet"}' --accountId test_did.testnet
```

6. 
```
near call test_did.testnet add_new_auth_key '{"pk":[0,2], "controller":"did:near:test_did.testnet"}' --accountId test_did.testnet
```

7. 
```
near call test_did.testnet add_context '{"context":["test_context"]}' --accountId test_did.testnet
```

8. 
```
near view test_did.testnet get_document '{"did":"did:near:test_did.testnet"}' --accountId test_did.testnet
```