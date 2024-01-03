mod balances;
mod system;

// Runtime
struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	// create a mutable runtime with genesis block
	let mut runtime = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	/* block #0 */
	// 1. set balance of alice as 100
	runtime.balances.set_balance(&alice, 100);
	// assert that the block is genesis.
	assert_eq!(runtime.system.block_number(), 0);

	/* block #1 */
	let _ = runtime.system.inc_block_number();
	// 1st tx: alice --101--> bob ❌ should fail, but won't panic.
	let _res = runtime
		.balances
		.transfer(&alice, &bob, 101)
		.map_err(|e| eprintln!("Error: {}", e));

	// 2nd tx: alice --20--> charlie ✅
	let _res = runtime
		.balances
		.transfer(&alice, &charlie, 20)
		.map_err(|e| eprintln!("Error: {}", e));

	// check the latest balance of all 3 - Alice, Bob, Charlie
	assert_eq!(runtime.balances.balance(&alice), 80);
	assert_eq!(runtime.balances.balance(&bob), 0);
	assert_eq!(runtime.balances.balance(&charlie), 20);
}
