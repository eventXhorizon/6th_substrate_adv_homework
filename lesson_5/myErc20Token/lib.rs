#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod my_erc20token {
    use ink::storage::Mapping;

    use trait_erc20::{TERC20, CustomError, Result};

    use ink::prelude::{
        vec,
        vec::Vec,
    };
    use ink::prelude::string::String;

    // 错误要被捕捉到，并且要被反馈到区块链外，所以必须符合一些条件
    // #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    // #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    // pub enum CustomError {
    //     BalanceTooLow,
    //     AllowanceTooLow,
    // }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance
    }

    #[ink(event)]
    pub struct Approve {
        from: AccountId,
        to: AccountId,
        value: Balance
    }

    // type Result<T> = core::result::Result<T, CustomError>;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MyErc20 {
        name: Vec<u8>,
        symbol: Vec<u8>,
        total_supply: Balance,      // #[ink::contract]会为我们引入一些默认的数据结构，Balance 就在其中
        balances: Mapping<AccountId, Balance>,
        allowance: Mapping<(AccountId, AccountId), Balance>
    }

    impl MyErc20 {

        #[ink(constructor)]
        pub fn new(name: Vec<u8>, symbol: Vec<u8>, total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            balances.insert(Self::env().caller(), &total_supply);

            Self::env().emit_event(
              Transfer {
                  from: None,
                  to: Some(Self::env().caller()),
                  value: total_supply
              }
            );

            Self {
                name,
                symbol,
                total_supply,
                balances,
                allowance: Default::default()
            }
        }

        #[ink(message)]
        pub fn get_name(&self) -> Vec<u8> {
            self.name.clone()
        }

        #[ink(message)]
        pub fn get_symbol(&self) -> Vec<u8> {
            self.symbol.clone()
        }

        // #[ink(message)]
        // pub fn total_supply(&self) -> Balance {
        //     self.total_supply
        // }
        //
        // #[ink(message)]
        // pub fn balance_of(&self, who: AccountId) -> Balance {
        //     self.balances.get(&who).unwrap_or_default()
        // }
        //
        // #[ink(message)]
        // pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
        //     let sender = self.env().caller();
        //
        //     return self.transfer_helper(&sender, &to, value);
        // }
        //
        // #[ink(message)]
        // pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
        //     let sender = self.env().caller();
        //     let mut allowance = self.allowance.get(&(from, sender)).unwrap_or_default();
        //
        //     if allowance < value {
        //         return Err(CustomError::AllowanceTooLow);
        //     }
        //
        //     self.allowance.insert(&(from, sender), &(allowance - value));
        //
        //     return self.transfer_helper(&from, &to, value);
        // }
        //
        // #[ink(message)]
        // pub fn approve(&mut self, to: AccountId, value: Balance) -> Result<()> {
        //     let sender = self.env().caller();
        //     self.allowance.insert(&(sender, to), &value);
        //
        //     self.env().emit_event(Approve {
        //         from: sender,
        //         to,
        //         value
        //     });
        //
        //     Ok(())
        // }

        pub fn transfer_helper(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<()> {
            let balance_from = self.balance_of(*from);
            let balance_to = self.balance_of(*to);

            if value > balance_from {
                return Err(CustomError::BalanceTooLow);
            }

            // 在 ink! 中，所有的 overflow 和 underflow 都会被自动处理，相当于 safe math，所以可以直接用减号
            self.balances.insert(from, &(balance_from - value));
            self.balances.insert(to, &(balance_to + value));

            self.env().emit_event(
                Transfer {
                    from: Some(*from),
                    to: Some(*to),
                    value
                }
            );

            Ok(())
        }
    }

    impl TERC20 for MyErc20 {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();

            return self.transfer_helper(&sender, &to, value);
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            let mut allowance = self.allowance.get(&(from, sender)).unwrap_or_default();

            if allowance < value {
                return Err(CustomError::AllowanceTooLow);
            }

            self.allowance.insert(&(from, sender), &(allowance - value));

            return self.transfer_helper(&from, &to, value);
        }

        #[ink(message)]
        fn approve(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            self.allowance.insert(&(sender, to), &value);

            self.env().emit_event(Approve {
                from: sender,
                to,
                value
            });

            Ok(())
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        // 参考展开后的写法
        type Event = <MyErc20 as ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn constructor_works() {
            let erc20 = MyErc20::new("my_token".as_bytes().to_vec(), "MTK".as_bytes().to_vec(), 10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.total_supply(), 10000);
            assert_eq!(erc20.balance_of(accounts.alice), 10000);    // 默认使用第一个账户，即 alice

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            let event = &emitted_events[0];

            // event 被 encode 过，想要知道原文的话，必须要 decode
            // Emitted_Event 的结构有两个字段，一个是 topics，一个是 data
            let decoded = <Event as scale::Decode>::decode(&mut &event.data[..]).expect("decoded error");

            match decoded {
                Event::Transfer(Transfer{ from, to, value }) => {
                    assert!(from.is_none(), "mint from error");
                    assert_eq!(to, Some(accounts.alice), "mint to error");
                    assert_eq!(value, 10000, "mint value error");
                },
                _ => panic!("match error")
            }
        }

        #[ink::test]
        fn transfer_should_works() {
            let mut erc20 = MyErc20::new("my_token".as_bytes().to_vec(), "MTK".as_bytes().to_vec(), 10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let res = erc20.transfer(accounts.bob, 12);

            assert!(res.is_ok());
            assert_eq!(erc20.balance_of(accounts.alice), 10000 - 12);
            assert_eq!(erc20.balance_of(accounts.bob), 12);
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            let mut erc20 = MyErc20::new("my_token".as_bytes().to_vec(), "MTK".as_bytes().to_vec(), 10000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // 设置当前环境的调用者
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);       // bob应该是没钱的
            let res = erc20.transfer(accounts.charlie, 12);
            assert!(res.is_err());
            assert_eq!(res, Err(CustomError::BalanceTooLow));
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Client 是模拟上链，发送交易的执行器

            let total_supply = 123;
            // 在每个合约生成的时候，都会有一个 Reference 合约，方便给其他合约调用
            let constructor  = Erc20Ref::new("my_token".as_bytes().to_vec(), "MTK".as_bytes().to_vec(), 10000);

            // 部署
            // 第一个参数是合约名
            // 第二个参数是部署人
            // 第三个参数是要执行的方法
            // 第四个参数是要转账的 value
            // 最后一个参数是 storage 相关的
            let contract_acc_id = client.instantiate(
                "erc20",
                &ink_e2e::alice(),
                constructor,
                0,
                None
            ).await
                .expect("instantiate failed")
                .account_id;                    // 这里表示如果方法执行成功，就取 account_id

            // ink_e2e::account_id 把 Keyring 变成 account_id
            let alice_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            // 发交易前先构造一笔 transfer msg，告诉链上交易内容长什么样。在构造的时候需要指定类型
            // 第一个参数是调用的地址
            // 调用的方法是一个闭包，并且闭包符合 Erc20Ref 的约束
            let transfer_msg = build_message::<Erc20Ref>(
                contract_acc_id.clone()
            ).call(|erc20| erc20.transfer(bob_acc, 2));

            // 构造完后，就把交易放到链上
            // 第一个参数是谁发起的交易
            // 第二个参数是交易长什么样
            // 第三个参数是 value
            // 第四个参数是 storage
            let res = client.call(&ink_e2e::alice(), transfer_msg, 0, None).await;

            assert!(res.is_ok());

            // 查看 balance 也是要跟链进行交互，所以也是要构造交易
            let balance_of_msg = build_message::<Erc20Ref>(contract_acc_id.clone())
                .call(|erc20| erc20.balance_of(alice_acc));
            // call_dry_run 表示不付费
            let balance_of_alice = client.call_dry_run(&ink_e2e::alice(), &balance_of_msg, 0, None).await;

            assert_eq!(balance_of_alice.return_value(), 121);   // 123 - 2

            Ok(())
        }
    }
}
