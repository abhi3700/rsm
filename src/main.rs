mod balances;
mod system;

// Runtime
#[derive(Debug)]
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

	// users
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();
	let den = "den".to_string();

	/* block #0 */
	// 1. set balance of alice as 100
	runtime.balances.set_balance(&alice, 100);
	println!("========================\n{:#?}\n========================", runtime);

	/* block #1 */
	let _ = runtime.system.inc_block_number();
	// 1st tx: alice --101--> bob ❌ should fail, but won't panic as the error is swallowed.
	match runtime.balances.transfer(&alice, &bob, 101) {
		Ok(_) => {
			let _ = runtime.system.inc_nonce(&alice);
		},
		Err(e) => eprintln!("Error: {}", e),
	};

	// 2nd tx: alice --20--> charlie ✅
	match runtime.balances.transfer(&alice, &charlie, 20) {
		Ok(_) => {
			let _ = runtime.system.inc_nonce(&alice);
		},
		Err(e) => eprintln!("Error: {}", e),
	};
	println!("========================\n{:#?}\n========================", runtime);

	/* block #2 */
	let _ = runtime.system.inc_block_number();
	// 1st tx: charlie --5--> bob ✅
	match runtime.balances.transfer(&charlie, &bob, 5) {
		Ok(_) => {
			let _ = runtime.system.inc_nonce(&charlie);
		},
		Err(e) => eprintln!("Error: {}", e),
	};

	// 2nd tx: bob --1--> den ✅
	match runtime.balances.transfer(&bob, &den, 1) {
		Ok(_) => {
			let _ = runtime.system.inc_nonce(&bob);
		},
		Err(e) => eprintln!("Error: {}", e),
	};

	// 3rd tx: den --1--> alice ✅
	match runtime.balances.transfer(&den, &alice, 1) {
		Ok(_) => {
			let _ = runtime.system.inc_nonce(&alice);
		},
		Err(e) => eprintln!("Error: {}", e),
	};
	println!("========================\n{:#?}\n========================", runtime);
}
