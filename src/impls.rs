use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::tokens::Preservation;
use frame::traits::Hash;

// Learn about internal functions.
impl<T: Config> Pallet<T> {
	// Generates and returns DNA and Sex
	pub fn gen_dna() -> [u8; 32] {
		// Create randomness payload. Multiple kitties can be generated in the same block,
		// retaining uniqueness.
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			CountForKitties::<T>::get(),
		);

		BlakeTwo256::hash_of(&unique_payload).into()
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		// let kitty = Kitty { dna, owner: owner.clone() };
		let kitty = Kitty { dna, owner: owner.clone(), price: None };

		// Check if the kitty does not already exist in our storage map
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		let current_count: u32 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;

		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyOwned)?;
		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(new_count);

		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}

	pub fn do_transfer(from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
		ensure!(from != to, Error::<T>::TransferToSelf);
		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		ensure!(kitty.owner == from, Error::<T>::NotOwner);
		kitty.owner = to.clone();

		let mut to_owned = KittiesOwned::<T>::get(&to);
		to_owned.try_push(kitty_id).map_err(|_| Error::<T>::TooManyOwned)?;
		let mut from_owned = KittiesOwned::<T>::get(&from);
		if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
			from_owned.swap_remove(ind);
		} else {
			return Err(Error::<T>::NoKitty.into())
		}

		Kitties::<T>::insert(kitty_id, kitty);
		KittiesOwned::<T>::insert(&to, to_owned);
		KittiesOwned::<T>::insert(&from, from_owned);

		/* 🚧 TODO 🚧: Sanity check the transfer is allowed:
			- First `ensure!` that `from` and `to` are not equal, else return `Error::<T>::TransferToSelf`.
			- Get the `kitty` from `Kitties` using `kitty_id`, else return `Error::<T>::NoKitty`.
			- Check the `kitty.owner` is equal to `from`, else return `NotOwner`.
		*/

		/* 🚧 TODO 🚧: Update the owner of the kitty:
			- Update `kitty.owner` to `to`.
			 - Update the `KittiesOwned` of `from` and `to:
				- Create a mutable `to_owned` by querying `KittiesOwned` for `to`.
				- `try_push` the `kitty_id` to the `to_owned` vector.
					- If the vector is full, `map_err` and return `Error::<T>::TooManyOwned`.
				- Create a mutable `from_owned` by querying `KittiesOwned` for `from`.
				- Write logic to `swap_remove` the item from the `from_owned` vector.
					- If you cannot find the kitty in the vector, return `Error::<T>::NoKitty`.
		*/

		/* 🚧 TODO 🚧: Update the final storage.
			- Insert into `Kitties` under `kitty_id` the modified `kitty` struct.
			- Insert into `KittiesOwned` under `to` the modified `to_owned` vector.
			- Insert into `KittiesOwned` under `from` the modified `from_owned` vector.
		*/

		Self::deposit_event(Event::<T>::Transferred { from, to, kitty_id });
		Ok(())
	}

	pub fn do_set_price(
		caller: T::AccountId,
		kitty_id: [u8; 32],
		new_price: Option<BalanceOf<T>>,
	) -> DispatchResult {
		/* 🚧 TODO 🚧: Create the logic for setting the Kitty price:
			- Create a mutable `kitty` by calling `get` on `Kitties` with `kitty_id`.
				- Return an error if the kitty doesn't exist by returning `Error::<T>::NoKitty`.
			- `ensure!` that the `kitty.owner` is equal to the `caller` else return `Error::<T>::NotOwner`.
			- Set the `kitty.price` to `new_price`.
			- Insert the modified `kitty` back into the `Kitties` map under `kitty_id`.
		*/

		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		ensure!(kitty.owner == caller, Error::<T>::NotOwner);
		kitty.price = new_price;
		Kitties::<T>::insert(kitty_id, kitty);

		Self::deposit_event(Event::<T>::PriceSet { owner: caller, kitty_id, new_price });
		Ok(())
	}

	pub fn do_buy_kitty(
		buyer: T::AccountId,
		kitty_id: [u8; 32],
		price: BalanceOf<T>,
	) -> DispatchResult {
		let kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		let real_price = kitty.price.ok_or(Error::<T>::NotForSale)?;
		ensure!(price >= real_price, Error::<T>::MaxPriceTooLow);

		T::NativeBalance::transfer(&buyer, &kitty.owner, real_price, Preservation::Preserve)?;
		Self::do_transfer(kitty.owner, buyer.clone(), kitty_id)?;

		Self::deposit_event(Event::<T>::Sold { buyer, kitty_id, price: real_price });
		Ok(())
	}
}
