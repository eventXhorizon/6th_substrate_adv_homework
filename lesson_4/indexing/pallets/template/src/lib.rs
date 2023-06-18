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

use sp_runtime::{
    offchain::{
        storage::{StorageValueRef},
    },
    traits::Zero,
};
use sp_io::offchain_index;

use serde::{Deserialize, Deserializer};
use sp_std::{str};
use sp_core::crypto::KeyTypeId;
use frame_system::{
    offchain::{
        AppCrypto, CreateSignedTransaction, SendSignedTransaction,
        Signer,
    },
};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocwd");
pub mod crypto {
    use super::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };
    app_crypto!(sr25519, KEY_TYPE);

    pub struct OcwAuthId;

    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for OcwAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for OcwAuthId
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
    use sp_std::vec;


    #[derive(Debug, Deserialize, Encode, Decode, Default)]
    // struct IndexingData(BoundedVec<u8>, u64);
    struct IndexingData(
        #[serde(deserialize_with = "de_string_to_bytes")]
        Vec<u8>,
        u64
    );

    pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
        where
            D: Deserializer<'de>
    {
        let s: &str = Deserialize::deserialize(de)?;
        Ok(s.as_bytes().to_vec())
    }

    const ONCHAIN_TX_KEY: &[u8] = b"ocw-demo::storage::tx";

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    }


    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
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
        pub fn submit_number_signed(origin: OriginFor<T>, payload: u64) -> DispatchResult {
            let key = Self::derive_key(frame_system::Module::<T>::block_number());
            let data = IndexingData(b"submit_number".to_vec(), payload);

            log::info!("EXTRINSIC ==> set key: {:?}", &key);
            // 使用 Offchain Indexing 从链上向 Offchain Storage 写入数据
            offchain_index::set(&key, &data.encode());

            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("OCW ==> Hello World from offchain workers!: {:?}", block_number);


            // Reading back the off-chain indexing value. It is exactly the same as reading from
            // ocw local storage.
            let key = Self::derive_key(block_number);
            let oci_mem = StorageValueRef::persistent(&key);

            log::info!("EXTRINSIC ==> set key: {:?}", str::from_utf8(&key).unwrap_or("error"),);

            let payload: u64 = 123;
            _ = Self::send_signed_tx(payload);

            if let Ok(Some(data)) = oci_mem.get::<IndexingData>() {
                log::info!(
                    "off-chain indexing data: {:?}, {:?}",
                    str::from_utf8(&data.0).unwrap_or("error"),
                    data.1)
                ;
            } else {
                log::info!("no off-chain indexing data retrieved.");
            }


            log::info!("OCW ==> Leave from offchain workers!: {:?}", block_number);
        }
    }

    impl<T: Config> Pallet<T> {
        #[deny(clippy::clone_double_ref)]
        fn derive_key(block_number: T::BlockNumber) -> Vec<u8> {
            block_number.using_encoded(|encoded_bn| {
                ONCHAIN_TX_KEY.clone()
                    .into_iter()
                    .chain(encoded_bn)
                    .copied()
                    .collect::<Vec<u8>>()
            })
        }

        fn send_signed_tx(payload: u64) -> Result<(), &'static str> {
            let signer = Signer::<T, T::AuthorityId>::all_accounts();
            if !signer.can_sign() {
                return Err(
                    "No local accounts available. Consider adding one via `author_insertKey` RPC.",
                )
            }

            let results = signer.send_signed_transaction(|_account| {
                Call::submit_number_signed { payload }
            });

            for (acc, res) in &results {
                match res {
                    Ok(()) => log::info!("[{:?}] Submitted data:{:?}", acc.id, payload),
                    Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
                }
            }

            Ok(())
        }
    }
}
