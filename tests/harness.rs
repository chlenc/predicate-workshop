use fuels::types::unresolved_bytes::UnresolvedBytes;
use fuels::{prelude::*, programs::script_calls::ScriptCallHandler};

abigen!(Predicate(
    name = "MyPredicate",
    abi = "out/debug/predicates-test-abi.json"
));

#[tokio::test]
async fn main_test() {
    let config = WalletsConfig::new(Some(3), Some(1), Some(10_000_000_000));
    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None)
        .await
        .unwrap();
    let alice = wallets.pop().unwrap();
    let bob = wallets.pop().unwrap();
    let chad = wallets.pop().unwrap();

    let predicate = Predicate::load_from("./out/debug/predicates-test.bin").unwrap();
    let provider = chad.provider().unwrap();
    let mut predicate = predicate.clone();
    predicate.set_provider(provider.clone());

    let predicate_root = predicate.address();
    println!("predicate_root = {:?}", predicate_root.to_string());

    let base_asset = AssetId::zeroed();

    let initial_chad_balance = chad.get_asset_balance(&base_asset).await.unwrap();
    alice
        .transfer(
            predicate_root,
            2 * 1e9 as u64,
            AssetId::zeroed(),
            TxPolicies::default(),
        )
        .await
        .unwrap();

    bob.transfer(
        predicate_root,
        3 * 1e9 as u64,
        AssetId::zeroed(),
        TxPolicies::default(),
    )
    .await
    .unwrap();

    let predicate_balance = predicate.get_asset_balance(&base_asset).await.unwrap();
    println!("predicate_balance = {:?}", predicate_balance);

    let mut inputs = vec![];
    let mut outputs = vec![];
    let mut output_to_maker =
        chad.get_asset_outputs_for_amount(chad.address(), base_asset, predicate_balance);
    outputs.append(&mut output_to_maker);

    let script_call = ScriptCallHandler::new(
        vec![],
        UnresolvedBytes::default(),
        chad.clone(),
        provider.clone(),
        Default::default(),
    )
    .with_inputs(inputs)
    .with_outputs(outputs)
    .with_tx_policies(TxPolicies::default().with_tip(1));

    script_call.call().await.unwrap();

    println!(
        "chad received from the predicate = {:?}",
        initial_chad_balance - chad.get_asset_balance(&base_asset).await.unwrap()
    );
}
