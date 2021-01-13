#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, sp_runtime, StorageMap, StorageValue,
    Parameter, traits::{Randomness, Currency, ReservableCurrency}
};
use frame_system::{ensure_signed};
use sp_io::hashing::blake2_128;
use sp_runtime::{
    DispatchError,
    traits::{AtLeast32BitUnsigned, Bounded}
};

//作业 2
// type KittyIndex = u32;

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
    type KittyIndex: Parameter + AtLeast32BitUnsigned + Bounded + Default + Copy;
    type Currency: ReservableCurrency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
const StakingMoney: u64 = 10;

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;
        pub KittiesCount get (fn kitties_count): T::KittyIndex;
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;

        //作业 3
        pub OwnedKittiesCount get(fn owned_kitty_count): map hasher(blake2_128_concat) T::AccountId => u64;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait>{
        KittiesCountOverFlow,
        InvalidKittyId,
        RequireDifferentParent,
        NotKittyOwner
    }
}

decl_event! {
    pub enum Event<T> where 
        <T as frame_system::Trait>::AccountId,
        KittyIndex = <T as Trait>::KittyIndex, {
        Created(AccountId, KittyIndex),
        Transferred(AccountId, AccountId, KittyIndex),
    }

}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 0]
        pub fn create(origin){
            let sender = ensure_signed(origin)?;

            //作业 3
            //获取账号里所有的猫咪数量
            let owned_kitty_count = Self::owned_kitty_count(&sender);
            let owner_all_kitties_count = owned_kitty_count.checked_add(1)
                .ok_or("kitties count overflow")?;

            let kitty_id = Self::next_kitty_id()?;
            let dna = Self::random_value(&sender);

            let kitty = Kitty(dna);

            T::Currency::reserve(&sender, StakingMoney)?;

            //作业 3
            <OwnedKittiesCount<T>>::insert(&sender, owner_all_kitties_count);

            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));
        }

        #[weight = 0]
        pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex){
            let sender = ensure_signed(origin)?;

            //转让kitty需要验证
            // 1.该转让的Kitty是否存在？

            let _ = Self::kitties(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;

            // 2.sender是否拥有Kitty？
            // let owner = Self::kitty_owner::get(kitty_id); Self写法怎么写？
            let owner = <KittyOwners<T>>::get(kitty_id);
            //解析Option获得kitty的主人

            ensure!(owner == sender, Error::<T>::NotKittyOwner);

            <KittyOwners<T>>::insert(kitty_id, to.clone());
            Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id))
        }

        #[weight = 0]
        pub fn breed(origin, kitty_id1: T::KittyIndex, kitty_id2: T::KittyIndex){
            let sender = ensure_signed(origin)?;
            let _new_kitty_id = Self::do_breed(&sender, kitty_id1, kitty_id2)?;
        }
    }
}
fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    (selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
    fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
        <Kitties<T>>::insert(kitty_id, kitty);
        <KittiesCount<T>>::put(kitty_id + 1.into());
        <KittyOwners<T>>::insert(kitty_id, owner);

    }

    fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        let kitty_id = Self::kitties_count();
        if kitty_id == T::KittyIndex::max_value() {
            return Err(Error::<T>::KittiesCountOverFlow.into());
        }
        Ok(kitty_id)
    }

    fn random_value(sender: &T::AccountId) -> [u8; 16] {
        let payload = (
            T::Randomness::random_seed(),
            &sender,
            <frame_system::Module<T>>::extrinsic_index(),
        );
        payload.using_encoded(blake2_128)
    }

    fn do_breed(
        sender: &T::AccountId,
        kitty_id_1: T::KittyIndex,
        kitty_id_2: T::KittyIndex,
    ) -> sp_std::result::Result<T::KittyIndex, DispatchError> {
        let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
        let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

        let kitty_id = Self::next_kitty_id()?;

        let kitty1_dna = kitty1.0;
        let kitty2_dna = kitty2.0;

        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        Self::insert_kitty(sender, kitty_id, Kitty(new_dna));
        Ok(kitty_id)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
