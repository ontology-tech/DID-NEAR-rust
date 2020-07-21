
1.
```
near login
```

2. 
```
near deploy --wasmFile output/UID_discovery_rust_optimized.wasm --accountId abcde.testnet
```

3. 
```
near call abcde.testnet reg_did_using_account --accountId abcdefg.testnet
```

4. 
```
ear view abcde.testnet get_did '{"id":"abcde.testnet"}' --accountId abcde.testnet
```