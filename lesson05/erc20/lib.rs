#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
// #[ink::contract(env = DefaultEnviorment)]
mod erc20 {

    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Erc20 {
        /// Stores a single `bool` value on the storage.
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowance: StorageHashMap<(AccountId, AccountId), Balance>,
        creator: AccountId,
    }
    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error{
        InSufficientBalance,
        NotCreator,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {

            let caller = Self::env().caller();

            let mut balances = StorageHashMap::new();
            balances.insert(caller, total_supply);

            let instence = Self {
                total_supply: total_supply,
                balances: balances,
                allowance: StorageHashMap::new(),
                creator: caller,
            };

            instence
        }

        //公共方法1，读  2.写
        #[ink(message)]
        pub fn total_supply(&self) ->Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self,owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        // #[ink(message)]
        // pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        //     self.allowances.get(&(owner, spender)).copied().unwrap_or(0)
        // }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()>{
            let who = Self::env().caller();
            self.transfer_helper(who, to, value)
        }

        //TODO
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>{
            self.transfer_helper(from, to, value)
        }

        //TODO
        //销毁total_supply的币数
        #[ink(message)]
        pub fn burn(&mut self, value: Balance) -> Result<()> {

            let caller = Self::env().caller();
            if caller != self.creator {
                return Err(Error::NotCreator)
            }
            let caller_balance = self.balance_of(caller);

            if caller_balance < value {
                return Err(Error::InSufficientBalance);
            }

            self.total_supply = self.total_supply() - value;
            self.balances.insert(caller, caller_balance - value);

            self.env().emit_event(Transfer {
                from: caller,
                to: AccountId::from([0x0; 32]),
                value: value,
            });
            Ok(())
        }

        //TODO
        //增发total_supply的币数
        #[ink(message)]
        pub fn issue(&mut self, value: Balance) -> Result<()>{

            let caller = Self::env().caller();
            if caller != self.creator {
                return Err(Error::NotCreator)
            }
            let caller_balance = self.balance_of(caller);

            self.total_supply = self.total_supply() + value;
            self.balances.insert(caller, caller_balance + value);

            Self::env().emit_event( Transfer{
                from: AccountId::from([0x0; 32]),
                to: caller,
                value: value
            });
            Ok(())
        }


        fn transfer_helper(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>{
            let from_balance = self.balance_of(from);

            if from_balance < value {
                return Err(Error::InSufficientBalance);
            }

            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);

            Self::env().emit_event( Transfer{
                from: from,
                to: to,
                value: value
            });
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn create_contract_works() {

            let erc20 = Erc20::new(1000);
            assert_eq!(erc20.total_supply(), 1000);
        }

    }
}
