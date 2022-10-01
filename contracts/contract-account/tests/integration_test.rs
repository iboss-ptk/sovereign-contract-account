mod helpers;
use helpers::with_env_setup;

use osmosis_testing::{cosmrs::proto::cosmos::feegrant::v1beta1 as feegrant, Account, Runner};

#[test]
fn signer_pays_no_fee() {
    with_env_setup(|app, _wasm, signer, _code_id, contract_account_addr| {
        let req = feegrant::QueryAllowanceRequest {
            granter: contract_account_addr,
            grantee: signer.address(),
        };

        let res: feegrant::QueryAllowanceResponse = app
            .query("/cosmos.feegrant.v1beta1.Query/Allowance", &req)
            .unwrap();

        dbg!(res);
    })
}
