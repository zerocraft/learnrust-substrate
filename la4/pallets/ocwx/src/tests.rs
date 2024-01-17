use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_core::offchain::{testing, OffchainWorkerExt, TransactionPoolExt};
//use sp_keystore::{testing::MemoryKeystore, KeystoreExt};

#[test]
fn should_submit_raw_unsigned_transaction_on_chain() {
	let (offchain, offchain_state) = testing::TestOffchainExt::new();
	let (pool, _pool_state) = testing::TestTransactionPoolExt::new();

	//let keystore = MemoryKeystore::new();

	let mut t = sp_io::TestExternalities::default();
	t.register_extension(OffchainWorkerExt::new(offchain));
	t.register_extension(TransactionPoolExt::new(pool));
	//t.register_extension(KeystoreExt::new(keystore));

	price_oracle_response(&mut offchain_state.write());

	t.execute_with(|| {
		// // when
		// OcwxModule::fetch_price_and_send_raw_unsigned(1).unwrap();
		// // then
		// let tx = pool_state.write().transactions.pop().unwrap();
		// assert!(pool_state.read().transactions.is_empty());
		// let tx = Extrinsic::decode(&mut &*tx).unwrap();
		// assert_eq!(tx.signature, None);
		// assert_eq!(
		// 	tx.call,
		// 	RuntimeCall::OcwxModule(crate::Call::submit_price_unsigned {
		// 		block_number: 1,
		// 		price: 15523
		// 	})
		// );
	});
}

fn price_oracle_response(state: &mut testing::OffchainState) {
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD".into(),
		response: Some(br#"{"USD": 155.23}"#.to_vec()),
		sent: true,
		..Default::default()
	});
}
