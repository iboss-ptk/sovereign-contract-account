use cosmwasm_std::Coin;

use contract_account::msg::InstantiateMsg;
use osmosis_testing::{Account, Module, OsmosisTestApp, SigningAccount, Wasm};
use std::path::PathBuf;

pub fn with_env_setup(
    run: impl Fn(&OsmosisTestApp, Wasm<OsmosisTestApp>, SigningAccount, u64, String),
) {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);
    let signer = app
        .init_account(&[
            Coin::new(100_000_000_000, "uosmo"),
            Coin::new(100_000_000_000, "uatom"),
        ])
        .unwrap();

    let code_id = wasm
        .store_code(&get_wasm_byte_code(), None, &signer)
        .unwrap()
        .data
        .code_id;
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                signer_addr: signer.address(),
            },
            None,
            None,
            &[],
            &signer,
        )
        .unwrap()
        .data
        .address;
    run(&app, wasm, signer, code_id, contract_addr)
}
pub fn get_wasm_byte_code() -> Vec<u8> {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    std::fs::read(
        manifest_path
            .join("..")
            .join("..")
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join("contract_account.wasm"),
    )
    .unwrap()
}
