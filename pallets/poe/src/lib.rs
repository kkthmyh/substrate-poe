#![cfg_attr(not(feature = "std"), no_std)]
mod mock;
mod tests;
// 存证模块
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
    use frame_system::ensure_signed;
    use sp_std::vec::Vec;

    /// 配置
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// 存储
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

    /// 事件
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransfered(T::AccountId, Vec<u8>),
    }

    /// error处理
    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        /// 创建存证
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // 判断是否是签名用户
            let sender = ensure_signed(origin)?;
            // 判断存证是否已经存在
            ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
            // 若不存在进行insert
            Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Pallet::<T>::block_number()));
            // 触发create事件
            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(().into())
        }

        #[pallet::weight(0)]
        /// 撤销存证
        pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // 判断是否是签名用户
            let sender = ensure_signed(origin)?;
            // 获取当前存证的所有者
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            // 只有所有者才可以撤销存证
            ensure!(owner == sender,Error::<T>::NotClaimOwner);
            // 撤销操作
            Proofs::<T>::remove(&claim);
            // 触发revoke事件
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }

        #[pallet::weight(0)]
        /// 转移存证
        pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, dest: T::AccountId) -> DispatchResultWithPostInfo {
            // 验证是否签名
            let sender = ensure_signed(origin)?;
            // 校验存证是否存在
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
            // 获取存证所有人
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            // 转移存证
            Proofs::<T>::insert(&claim, (dest, frame_system::Module::<T>::block_number()));

            Ok(().into())
        }
    }
}
