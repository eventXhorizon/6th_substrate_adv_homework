
use frame_support::{
    pallet_prelude::*,
    storage::StoragePrefixedMap,
    traits::GetStorageVersion,
    weights::Weight,
};

use frame_system::pallet_prelude::*;
use frame_support::{migration::storage_key_iter, Blake2_128Concat};

use crate::*;

// 在升级前需要知道旧的数据结构是什么样的
#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct V0Kitty(pub [u8; 16]);

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct V1Kitty {
    pub dna: [u8; 16],
    pub name: [u8; 4]
}

pub fn migrate<T: Config>() -> Weight {

    let on_chain_version = Pallet::<T>::on_chain_storage_version();

    let current_version = Pallet::<T>::current_storage_version();

    if on_chain_version != 0 {
        return Weight::zero();
    }

    if current_version != 1 {
        return Weight::zero();
    }

    if on_chain_version == 0 {
        from_v0_to_v2::<T>();
    }

    if on_chain_version == 1 {
        from_v1_to_v2::<T>();
    }

    Weight::zero()
}

pub fn from_v0_to_v2<T:Config>() {
    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();

    for (index, kitty) in storage_key_iter::<KittyId, V0Kitty, Blake2_128Concat>(module, item).drain() {
        let new_kitty = Kitty {
            dna: kitty.0,
            name: *b"abcdxxxx",
        };
        Kitties::<T>::insert(index, &new_kitty);
    }
}

pub fn from_v1_to_v2<T:Config>() {
    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();

    for (index, kitty) in storage_key_iter::<KittyId, V1Kitty, Blake2_128Concat>(module, item).drain() {
        let v2_kitty = Kitty { name: rename(&kitty.name, b"0987"), dna: kitty.dna };
        Kitties::<T>::insert(index, &v2_kitty);
    }
}

fn rename(v1_name: &[u8; 4], append: &[u8; 4]) -> [u8; 8] {
    let mut result = [0; 8];
    result[..4].copy_from_slice(v1_name);
    result[4..].copy_from_slice(append);
    result
}