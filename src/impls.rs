use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::Hash;

impl<T: Config> Pallet<T> {
	pub fn gen_dna() -> [u8; 32] {
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			CountForKitties::<T>::get(),
		);

		BlakeTwo256::hash_of(&unique_payload).into()
	}

	// pub fn mint(owner: T::AccountId) -> DispatchResult {
	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty = Kitty { dna, owner: owner.clone() };

		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		// let current_count: u32 = CountForKitties::<T>::get().unwrap_or(0);
		let current_count: u32 = CountForKitties::<T>::get();

		// let new_count = current_count + 1;
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		// KittiesOwned::<T>::append(&owner, dna);
		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyOwned)?;
		// Kitties::<T>::insert(dna, ());
		Kitties::<T>::insert(dna, kitty);
		// CountForKitties::<T>::set(Some((new_count)));
		CountForKitties::<T>::set(new_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
