use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &String, balance: u128) {
		self.balances.insert(who.clone(), balance);
	}

	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(
		&mut self,
		from: &String,
		to: &String,
		amount: u128,
	) -> Result<(), &'static str> {
		// get the old balances
		let from_old_balance = self.balance(from);
		let to_old_balance = self.balance(to);

		// calculate the new balances
		let from_new_balance =
			from_old_balance.checked_sub(amount).ok_or("Insufficient balance")?;
		let to_new_balance = to_old_balance.checked_add(amount).ok_or("Exceeds MAX balance")?;

		// set the new balances
		self.set_balance(from, from_new_balance);
		self.set_balance(to, to_new_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn test_transfer() {
		let mut balances = super::Pallet::new();

		assert_eq!(
			balances.transfer(&"alice".to_string(), &"bob".to_string(), 100),
			Err("Insufficient balance")
		);

		balances.set_balance(&"alice".to_string(), 100);
		assert!(balances.transfer(&"alice".to_string(), &"bob".to_string(), 51).is_ok());

		// check the balances
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);
	}
}
