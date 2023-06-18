
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
pub struct OldKitty(pub [u8; 16]);

pub fn migrate<T: Config>() -> Weight {
    // 首先需要得到两个不同的版本号

    // 旧的版本号
    let on_chain_version = Pallet::<T>::on_chain_storage_version();

    // 获取新版本的常量
    let current_version = Pallet::<T>::current_storage_version();

    // 我们希望在 v1 里定义的 migration 只适用于从版本0到版本1的升级
    if on_chain_version != 0 {
        return Weight::zero();
    }

    if current_version != 1 {
        return Weight::zero();
    }

    // 得到前缀
    /*  storage 是通过一组变量，如 pallet 名字/前缀，storage 的名字/前缀组合在一起。然后再根据数据类型，例如一个map类型
        会把 key 和前面两个东西组合在一起，作为最终的数据库里的 key
    */
    // 拿到最前面的前缀
    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();

    // 遍历所有 kitty，并且清理掉旧的数据。单纯遍历的话用 enumerate
    for (index, kitty) in storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, item).drain() {
        let new_kitty = Kitty {
            dna: kitty.0,
            name: *b"abcd",
        };
        Kitties::<T>::insert(index, &new_kitty);
    }

    Weight::zero()
}