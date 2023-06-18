#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_system::pallet_prelude::*;            // 需要用到签名
    use frame_support::pallet_prelude::*;
    use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        // 下面这行注释可以在前端展示
        /// The maximum length of claim that can be added.
        #[pallet::constant]                     // 声明这是一个常量
        type MaxClaimLength: Get<u32>;          // 存证的最大长度限制。如果太长会导致链上状态爆炸

        // 在Runtime进行配置接口实现时，把Runtime定义的Event设置在这个类型里
        // 当我们所要求的Event满足这些条件，就可以从当前模块Event类型转换过去
        // 同时它是系统模块的Event类型
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }


    // 模块结构体
    #[pallet::pallet]                                   // 需要加 #[pallet::pallet] 宏
    #[pallet::generate_store(pub(crate) trait Store)]   // 要定义自己的存储项，需要generate_store 宏，来生成包含所有存储项的 trait Store 接口
    pub struct Pallet<T>(_);

    // 定义存储项
    // 不能直接使用Vec，只能使用 BoundedVec 这种长度受限的安全的集合
    #[pallet::storage]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, T::BlockNumber),
    >;

    // 定义事件
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]    // 用 generate_deposit 宏，声明了一个 deposit_event 方法，方便生成事件
    pub enum Event<T: Config> {
        // ClaimCreated(T::AccountId, Vec<u8>),
        // ClaimRevoked(T::AccountId, Vec<u8>),

        ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
        ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
    }

    // 定义错误信息
    #[pallet::error]
    pub enum Error<T> {
        /// 存证已经存在，已经被创建
        ProofAlreadyExist,
        /// 存证过长
        ClaimTooLong,
        /// 存证不存在，无法撤销
        ClaimNotExist,
        /// 该存证是由另外一个用户创建，当前账户无权处理
        NotClaimOwner,
    }

    // 定义保留的函数，这些函数一般在区块的不同时机执行
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // 这里没有想要定义的特殊函数，所以为空
    }

    // 定义可调用函数
    #[pallet::call]                 // 使用 #[pallet::call] 宏来定义可调用函数
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]        // 具体的权重应该要由 benchmark 来测试得出
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
        // pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo {
            // origin 是交易发送方，claim 是存证的内容
            let sender = ensure_signed(origin)?;

            // 检查claim是否超过最大长度.把类型为 Vec<u8> 的 claim 转换为 BoundedVec。如果转换出错，就返回 ClaimTooLong
            let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
                    .map_err(|_| Error::<T>::ClaimTooLong)?;

            // 检查存储项不包括 bounded_claim 变量所代表的 key
            ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);
            // ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            // 存入
            Proofs::<T>::insert(
                &bounded_claim,
                // &claim,
                // 第一个元素是发送者（存证的Owner）,第二个元素是当前区块
                (sender.clone(), frame_system::Pallet::<T>::block_number()),
            );

            // 插入成功，触发事件
            Self::deposit_event(Event::ClaimCreated(sender, bounded_claim));
            // Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }

        // 删除存证
        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
        // pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
                .map_err(|_| Error::<T>::ClaimTooLong)?;

            let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
            // let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&bounded_claim);
            // Proofs::<T>::remove(&claim);

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, dest: T::AccountId) -> DispatchResultWithPostInfo {
        // pub fn transfer_claim(origin: OriginFor<T>, claim: BoundedVec<u8, T::MaxClaimLength>, dest: T::AccountId) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
                .map_err(|_| Error::<T>::ClaimTooLong)?;

            let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
            // let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::insert(
                &bounded_claim,
                // &claim,
                (dest.clone(), frame_system::Pallet::<T>::block_number()));

            Ok(().into())
        }
    }
}