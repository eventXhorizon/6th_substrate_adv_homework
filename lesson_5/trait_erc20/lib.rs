#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::env::*;

pub type Result<T> = core::result::Result<T, CustomError>;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomError {
    BalanceTooLow,
    AllowanceTooLow,
}

// AccountId 和 Balance 都是 #[ink::contract] 自动引入的，#[ink::trait_definition] 不会帮我们自动引入
// 所以要手动引入，具体写法参考展开后的文件
// type Environment = <Erc20 as ink::env::ContractEnv>::Env;
// type AccountId = <<Erc20 as ink::env::ContractEnv>::Env as ink::env::Environment>::AccountId;
// type Balance = <<Erc20 as ink::env::ContractEnv>::Env as ink::env::Environment>::Balance;

type Environment = DefaultEnvironment;     // Environment 一般是用 DefaultEnvironment
type AccountId = <DefaultEnvironment as ink::env::Environment>::AccountId;
type Balance = <DefaultEnvironment as ink::env::Environment>::Balance;


#[ink::trait_definition]
pub trait TERC20 {
    #[ink(message)]
    fn balance_of(&self, who: AccountId) -> Balance;

    #[ink(message)]
    fn total_supply(&self) -> Balance;

    #[ink(message)]
    fn approve(&mut self, to: AccountId, value: Balance) -> Result<()>;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()>;

    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>;
}
