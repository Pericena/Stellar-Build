# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
├── AGENTS.md
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.


CONTRATO:CCCLDHPCME7XHSGXK6LJHE7TGWLNGH5WNXXI3GHIXXQRIRE2JW2PTAAR



C:\stellar\stellar-event-pass>stellar contract invoke --id CCCLDHPCME7XHSGXK6LJHE7TGWLNGH5WNXXI3GHIXXQRIRE2JW2PTAAR --source-account eventpass --network testnet -- buy_pass --user GD26UBYVEYYVVOVCMOLPMIKPWQRFV34LK3I7LHBNTUGYHYIKFMEREH2A
ℹ️  Simulating transaction…
ℹ️  Signing transaction: 6be268a284eee59916485c32eadc2d89d092c1b14c60443969cb504996147181
🌎 Sending transaction…
✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/6be268a284eee59916485c32eadc2d89d092c1b14c60443969cb504996147181


>stellar contract invoke --id CCCLDHPCME7XHSGXK6LJHE7TGWLNGH5WNXXI3GHIXXQRIRE2JW2PTAAR --source-account eventpass --network testnet -- --help
Usage: stellar contract invoke --id CCCLDHPCME7XHSGXK6LJHE7TGWLNGH5WNXXI3GHIXXQRIRE2JW2PTAAR --source-account eventpass --network testnet -- [COMMAND]

Commands:
  is_used
  buy_pass
  has_pass
  use_pass
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
  



C:\stellar\stellar-event-pass>stellar contract invoke --id CCCLDHPCME7XHSGXK6LJHE7TGWLNGH5WNXXI3GHIXXQRIRE2JW2PTAAR --source-account eventpass --network testnet -- --help
Usage: stellar contract invoke --id CCCLDHPCME7XHSGXK6LJHE7TGWLNGH5WNXXI3GHIXXQRIRE2JW2PTAAR --source-account eventpass --network testnet -- [COMMAND]

Commands:
  is_used
  buy_pass
  has_pass
  use_pass
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help


C:\stellar\stellar-event-pass>



stellar contract deploy --source-account eventpass --network testnet --alias eventpass
ℹ️  CARGO_BUILD_RUSTFLAGS=--remap-path-prefix=C:/Users/Brigith/.cargo/registry/src= SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2=1 cargo rustc --manifest-path=contracts\hello-world\Cargo.toml --crate-type=cdylib --target=wasm32v1-none --release
    Finished `release` profile [optimized] target(s) in 1.80s
ℹ️  Build Summary:
    Wasm File: target\wasm32v1-none\release\hello_world.wasm (927 bytes optimized (original size was 1036 bytes))
    Wasm Hash: 13f466c5972b343c534ff22ee34f1c83da0a196b8aed80965c591b9664b8a5ae
    Wasm Size: 927 bytes optimized (original size was 1036 bytes)
    Exported Functions: 4 found
      • buy_pass
      • has_pass
      • is_used
      • use_pass
✅ Build Complete

ℹ️  Uploading contract WASM…
ℹ️  Simulating transaction…
ℹ️  Signing transaction: b6f5458bf201bfba9d76c8458f7e3ea4d8cfccbacb24258c841326cde258bbfd
🌎 Sending transaction…
✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/b6f5458bf201bfba9d76c8458f7e3ea4d8cfccbacb24258c841326cde258bbfd
ℹ️  Deploying contract using wasm hash 13f466c5972b343c534ff22ee34f1c83da0a196b8aed80965c591b9664b8a5ae
ℹ️  Simulating transaction…
ℹ️  Signing transaction: 3192fe4ea56fb8e8decbbae92ab69955f151355858cb6f24032555ddf7f4ae2e
🌎 Sending transaction…
✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/3192fe4ea56fb8e8decbbae92ab69955f151355858cb6f24032555ddf7f4ae2e
🔗 https://lab.stellar.org/r/testnet/contract/CCJOMWCF3UI4Z66PP7KINJF4I7IHLHVLWUVETC4BHBLIBATLMEFLUC2S
✅ Deployed!
❌ error: Contract alias eventpass, cannot overlap with key