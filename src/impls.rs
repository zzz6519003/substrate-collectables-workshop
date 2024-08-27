use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	// pub fn mint(owner: T::AccountId) -> DispatchResult {
	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		// let current_count: u32 = CountForKitties::<T>::get().unwrap_or(0);
		let current_count: u32 = CountForKitties::<T>::get();

		// let new_count = current_count + 1;
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		Kitties::<T>::insert(dna, ());
		// CountForKitties::<T>::set(Some((new_count)));
		CountForKitties::<T>::set(new_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
