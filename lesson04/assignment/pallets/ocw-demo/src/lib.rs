//! A demonstration of an offchain worker that sends onchain callbacks

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

use core::{convert::TryInto};
use frame_support::{
	debug, decl_error, decl_event, decl_module, decl_storage
};
use frame_system::{
	self as system,
	ensure_signed,
	offchain::{
		AppCrypto, CreateSignedTransaction, Signer,
	},
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	offchain as rt_offchain,
};
use serde_json::{Value};
use sp_std::{collections::vec_deque::VecDeque, prelude::*, str};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");
pub const NUM_VEC_LEN: usize = 10;
pub const HTTP_REMOTE_REQUEST: &str = "https://api.coincap.io/v2/assets/polkadot";
pub const FETCH_TIMEOUT_PERIOD: u64 = 3000; 

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrapper.
/// We can utilize the supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// them with the pallet-specific identifier.
pub mod crypto {
	use crate::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	use sp_runtime::{traits::Verify, MultiSignature, MultiSigner};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;
	// implemented for ocw-runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}


/// This is the pallet's configuration trait
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Example {
		pub DotPrices get(fn dot_prices): VecDeque<Option<Vec<u8>>>;
	}
}

decl_event!(
	/// Events generated by the module.
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		/// Event generated when a new number is accepted to contribute to the average.
		NewPrices(AccountId, Vec<u8>),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		// Error returned when not sure which ocw function to executed
		UnknownOffchainMux,

		// Error returned when making signed transactions in off-chain worker
		NoLocalAcctForSigning,
		OffchainSignedTxError,

		// Error returned when making unsigned transactions in off-chain worker
		OffchainUnsignedTxError,

		// Error returned when making unsigned transactions with signed payloads in off-chain worker
		OffchainUnsignedTxSignedPayloadError,

		// Error returned when fetching github info
		HttpFetchingError,
		FetchInfoError,
		FetchPriceError
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10000]
		pub fn submit_dot_price(origin, price: Vec<u8>){
			//TODO , update the dot price vec

			let sender = ensure_signed(origin)?;

			DotPrices::mutate(|prices| {
				if prices.len() == NUM_VEC_LEN {
					let _ = prices.pop_front();
				}
				prices.push_back(Some(price.clone()));
			});

			Self::deposit_event(RawEvent::NewPrices(sender, price));
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain worker");

			const TX_TYPES: u32 = 4;

			//获取当前height除以4的余数
			let modu = block_number.try_into().map_or(TX_TYPES, |bn: usize| (bn as u32) % TX_TYPES);
			let result = match modu {
				3 => Self::offchain_signed(),
				_ => Err(Error::<T>::UnknownOffchainMux),
			};

			if let Err(e) = result {
				debug::error!("off-chain worker error: {:?}", e);
			}
		}
	}
}

impl<T: Trait> Module<T> {
	fn offchain_signed() -> Result<(), Error<T>> {
		let signer_own = Signer::<T, T::AuthorityId>::any_account();

		let priceData: Vec<u8> = Self::fetch_dot_price().map_err(|_| <Error<T>>::FetchPriceError)?;

		let info2Str = str::from_utf8(&priceData).map_err(|_| <Error<T>>::FetchInfoError)?;
		debug::info!("=============info string is  {} ============", info2Str);

		//formate info
		let asset: Value = serde_json::from_str(&info2Str).map_err(|_| <Error<T>>::HttpFetchingError)?;
		let price : Vec<u8> = asset["data"]["priceUsd"].as_str().unwrap().as_bytes().to_vec();

		debug::info!("price is {:?}", price);

		let result = signer_own.send_signed_transaction(|_acct|
			// This is the on-chain function
			Call::submit_dot_price(price.clone())
		);

		Ok(())
	}

	//fetch dot price
	fn fetch_dot_price() -> Result<Vec<u8>, Error<T>> {
		debug::info!("do request fetch...");

		let request = rt_offchain::http::Request::get(HTTP_REMOTE_REQUEST);

		let timeout = sp_io::offchain::timestamp()
			.add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

		let pending = request
			.deadline(timeout)
			.send()
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		let response = pending
			.try_wait(timeout)
			.map_err(|_| <Error<T>>::HttpFetchingError)?
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		if response.code != 200 {
			debug::error!("http request fail, error code is {} ", response.code);
			return Err(<Error<T>>::HttpFetchingError);
		}

		Ok(response.body().collect::<Vec<u8>>())
	}
}