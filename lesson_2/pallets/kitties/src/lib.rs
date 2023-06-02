#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // 引入能产生128位随时数的trait
    use sp_io::hashing::blake2_128;
    // 产生一个128位的值
    // 让产生的值随机。因为我们不会直接在 pallet 里用某个方法产生一个随机的值。
    // 而是可以在runtime里任意绑定一个数据结构或者方法，然后通过这个trait，产生一个随机值
    use frame_support::traits::Randomness;

    pub type KittyId = u32;

    // 在链上存储的数据需要满足一特征
    #[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
    pub struct Kitty(pub [u8; 16]);


    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // /// Type representing the weight of this pallet
        // type WeightInfo: WeightInfo;

        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

    // 创建下一个kitty的时候，id应该是多少
    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]        // 给一个get方法
    // StorageValue<Prefix, Value, QueryKind, OnEmpty>	ValueQuery 给定了一个Value的类型，如果是u32的话，那么就是0
    pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;

    // 存储 kitty 这个 struct
    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

    // kitty 的 owner，可以直接查询 owner 是谁，然后在 transfer 的时候可以直接 check
    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_parents)]
    // pub struct StorageMap<Prefix, Hasher, Key, Value, QueryKind = OptionQuery, OnEmpty = GetDefault, MaxValues = GetDefault>(_);
    pub type KittyParents<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId), OptionQuery>;


    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        KittyCreated { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },

        KittyBreed { who: T::AccountId, kitty_id: KittyId, kitty: Kitty },

        KittyTransferred { who: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        InvalidKittyId,
        SameKittyId,
        NotOwner,
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        // #[pallet::weight(T::WeightInfo::do_something())]
        #[pallet::weight(10_000)]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let kitty_id = Self::get_next_id()?;

            // 为 kitty 产生一个随机数据
            let kitty = Kitty(Self::random_value(&who));

            Kitties::<T>::insert(kitty_id, &kitty);
            KittyOwner::<T>::insert(kitty_id, &who);

            Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn breed(origin: OriginFor<T>, kitty_id_1: KittyId, kitty_id_2: KittyId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

            // 判断父母 kitty 是否都存在
            ensure!(Kitties::<T>::contains_key(kitty_id_1), Error::<T>::InvalidKittyId);
            ensure!(Kitties::<T>::contains_key(kitty_id_2), Error::<T>::InvalidKittyId);

            let kitty_id = Self::get_next_id()?;

            // 把父母 kitty 取出来
            let kitty_1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
            let kitty_2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

            // 产生一个新的 kitty
            // 这样做的结果是新的 kitty 的数据其实都是来源于 parents
            let selector = Self::random_value(&who);
            let mut data = [0u8; 16];
            for i in 0..kitty_1.0.len() {
                // 0 choose kitty2, and 1 choose kitty1
                // 用随机数与 kitty 的每一个位进行位运算
                // !selector[i]：对 u8 数据使用 ! 运算符会将其视为一个位运算符，对其进行按位取反操作
                data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
            }
            let kitty = Kitty(data);

            // 链上数据更新
            Kitties::<T>::insert(kitty_id, &kitty);
            KittyOwner::<T>::insert(kitty_id, &who);
            KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));

            Self::deposit_event(Event::KittyBreed { who, kitty_id, kitty });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn transfer(origin: OriginFor<T>, recipient: T::AccountId, kitty_id: KittyId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(KittyOwner::<T>::contains_key(kitty_id), Error::<T>::NotOwner);

            let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
            ensure!(owner == who, Error::<T>::NotOwner);

            KittyOwner::<T>::insert(kitty_id, &recipient);
            Self::deposit_event(Event::KittyTransferred { who, recipient, kitty_id });

            Ok(())
        }
    }

    // 得到当前id的时候，对 kittyId 进行自增操作。由于这个方法不是一个call，只是一个纯函数，所以是直接定义在外面
    impl<T: Config> Pallet<T> {
        fn get_next_id() -> Result<KittyId, DispatchError> {
            // 当超过u32范围时失败
            // Mutate the value if closure returns Ok
            NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
                let current_id = *next_id;
                *next_id = next_id.checked_add(1).ok_or::<DispatchError>(Error::<T>::InvalidKittyId.into())?;
                Ok(current_id)
            })
        }

        // 产生128位随机数
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            // 把下面3个参数绑定在一起就可以保证 payload 的唯一性
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );

            // 保证位数符合要求
            // using_encoded 方法是在 Encode trait 下面定义的
            // impl<T, const N: usize> Encode for [T; N]，所以[u8; 16]可以直接用 using_encoded 方法
            payload.using_encoded(blake2_128)
        }
    }
}

