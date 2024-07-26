predicate;

use std::constants::ZERO_B256;
use std::inputs::{input_amount, input_asset_id};
use std::outputs::{output_asset_id, output_asset_to};

configurable {
    FUNDRAISER_ADDRESS: Address = Address::from(ZERO_B256),
    FUNDRAISER_ASSET: b256 = ZERO_B256,
    GOAL_AMOUNT: u64 = 5,
}

fn main() -> bool {
    assert(input_amount(0).unwrap() >= GOAL_AMOUNT);
    assert(output_asset_id(0).unwrap() == AssetId::from(FUNDRAISER_ASSET));
    assert(output_asset_to(0).unwrap() == FUNDRAISER_ADDRESS);
    true
}