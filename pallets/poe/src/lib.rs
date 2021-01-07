/*
	分为6个部分
*/

#![cfg_attr(not(feature = "std"), no_std)]
// 1.import

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch };
use frame_system::ensure_signed;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// 2.config
pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// 3.decl_storage!()
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// 4.decl_event!()
decl_event! {
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
	}
}

// 5.decl_error!()
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
	}
}

// 6.decl_module!()
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin{
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10_000]
		fn create_claim(origin, proof: Vec<u8>){
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			let current_block = <frame_system::Module<T>>::block_number();

			Proofs::<T>::insert(&proof, (&sender, current_block));

			Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
		}


		#[weight = 10_000]
		fn revoke_claim(origin, proof: Vec<u8>){
			let sender = ensure_signed(origin)?;

			ensure!(Proofs:: <T>::contains_key(&proof), Error::<T>::NoSuchProof);

			let (owner, _) = Proofs::<T>::get(&proof);

			ensure!(sender == owner, Error::<T>::NotProofOwner);

			Proofs::<T>::remove(&proof);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));

		}

		#[weight = 10_000]
		fn transfer_claim(origin, claim: Vec<u8>, dest: T::AccountId) -> dispatch::DispatchResult {

			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NoSuchProof);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotProofOwner);

			Proofs::<T>::insert(&claim, (dest, frame_system::Module::<T>::block_number()));

			Ok(())
		}
	}

}
