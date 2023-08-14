# icp_rust_message_board_contract

### Requirements
```
rust 1.64 or higher
wasmtime (https://docs.wasmtime.dev/cli-install.html)
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
DFX_VERSION=0.14.3 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
cd icp_rust_message_board_contract/
dfx help
dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.3"
ic-cdk = "0.10.0"
serde = { version = "1", features = ["derive"] }
serde_cbor = "0.10"
ic-cdk-macros = "0.7.1"
uuid = { version = "1.4.1", features = ["v4", "serde", "wasm-bindgen"] }
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid. 
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```
