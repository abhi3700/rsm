//! System pallet

use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) -> Result<(), &'static str> {
		self.block_number = self.block_number.checked_add(1).ok_or("Exceeded u32 MAX")?;

		Ok(())
	}

	pub fn inc_nonce(&mut self, who: &AccountId) -> Result<(), &'static str> {
		let new_nonce =
			self.nonce.get(who).unwrap_or(&0).checked_add(1).ok_or("Exceeded u32 MAX")?;

		self.nonce.insert(who.clone(), new_nonce);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::new();

		// get the genesis block number
		assert_eq!(system.block_number(), 0);
		// view alice's nonce at the genesis block
		assert_eq!(system.nonce.get(&"alice".to_string()), None);

		// inc block & nonce of alice
		assert!(system.inc_block_number().is_ok());
		assert!(system.inc_nonce(&"alice".to_string()).is_ok());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
		assert_eq!(system.nonce.get(&"bob".to_string()), None);
	}
}
