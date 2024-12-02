use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let current_count = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		CountForKitties::<T>::set(new_count);
		Kitties::<T>::insert(dna, ());
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
