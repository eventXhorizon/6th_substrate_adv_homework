#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weather;

use frame_system::{
    offchain::{
        AppCrypto, CreateSignedTransaction, SendUnsignedTransaction,
        SignedPayload, Signer, SigningTypes,
    },
};

use sp_runtime::{
    offchain::{
        http, Duration,
    },
    transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
    RuntimeDebug,
};

use codec::{Decode, Encode};

use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");
pub mod crypto {
    use super::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };
    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;

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

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

    // 定义 Payload
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
    pub struct Payload<Public> {
        // number: u64,
        temp: u64,
        public: Public,		// sender 账户的公钥
    }

    impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public> {
        fn public(&self) -> T::Public {
            self.public.clone()
        }
    }

    
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn unsigned_extrinsic_with_signed_payload(origin: OriginFor<T>, payload: Payload<T::Public>, _signature: T::Signature,) -> DispatchResult {
            ensure_none(origin)?;

            log::info!("OCW ==> in call unsigned_extrinsic_with_signed_payload: {:?}", payload.temp);
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

	}

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            const UNSIGNED_TXS_PRIORITY: u64 = 100;
            let valid_tx = |provide| ValidTransaction::with_tag_prefix("my-pallet")
                .priority(UNSIGNED_TXS_PRIORITY) // please define `UNSIGNED_TXS_PRIORITY` before this line
                .and_provides([&provide])
                .longevity(3)
                .propagate(true)
                .build();

            // match call {
            // 	Call::submit_data_unsigned { key: _ } => valid_tx(b"my_unsigned_tx".to_vec()),
            // 	_ => InvalidTransaction::Call.into(),
            // }

            match call {
                Call::unsigned_extrinsic_with_signed_payload {
                    ref payload,
                    ref signature
                } => {
                    if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
                        return InvalidTransaction::BadProof.into();
                    }
                    valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
                },
                _ => InvalidTransaction::Call.into(),
            }
        }
    }

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("OCW ==> Hello World from offchain workers!: {:?}", block_number);

            let mut temp: f64 = 0.0;

            if let Ok(info) = Self::fetch_weather_info() {
                log::info!("OCW ==> Weather Info: {:?}", info);
                log::info!("OCW ==> temp: {:?}", info.main.temp);
                temp = info.main.temp;
            } else {
                log::info!("OCW ==> Error while fetch weather!");
            }

            let signer = Signer::<T, T::AuthorityId>::any_account();

            if let Some((_, res)) = signer.send_unsigned_transaction(
                // this line is to prepare and return payload
                |acct| Payload { temp: temp as u64, public: acct.public.clone() },
                |payload, signature| Call::unsigned_extrinsic_with_signed_payload { payload, signature },
            ) {
                match res {
                    Ok(()) => {log::info!("OCW ==> unsigned tx with signed payload successfully sent.");}
                    Err(()) => {log::error!("OCW ==> sending unsigned tx with signed payload failed.");}
                };
            } else {
                // The case of `None`: no account is available for sending
                log::error!("OCW ==> No local account available");
            }

         	log::info!("OCW ==> Leave from offchain workers!: {:?}", block_number);
        }
    }

    impl<T: Config> Pallet<T> {
        // fn fetch_weather_info() -> Result<GithubInfo, http::Error> {
        fn fetch_weather_info() -> Result<weather::All, http::Error> {
            // prepare for send request
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
            let request =
                // http::Request::get("https://api.github.com/orgs/substrate-developer-hub");
                http::Request::get("https://api.openweathermap.org/data/2.5/weather?q=Beijing&appid=4845f22236e074cdac59ae174aa580a3");
            let pending = request
                .add_header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/112.0")
                .deadline(deadline).send().map_err(|_| http::Error::IoError)?;
            let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
            if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown)
            }
            let body = response.body().collect::<Vec<u8>>();
            let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
                log::warn!("No UTF8 body");
                http::Error::Unknown
            })?;
            log::info!("body_str: {}", body_str);

            // parse the response str
            let weather_info: weather::All =
                serde_json::from_str(body_str).map_err(|_| http::Error::Unknown)?;
            log::info!("weather_info: {:?}", weather_info);
            Ok(weather_info)
        }

    }
}
