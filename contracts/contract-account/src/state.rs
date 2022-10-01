use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const SIGNER: Item<Addr> = Item::new("signer");
