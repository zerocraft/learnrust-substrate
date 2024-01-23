#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BalanceTooLow,
        AllowanceTooLow,
    }

    type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::new();
            balances.insert(Self::env().caller(), &total_supply);
            Self {
                total_supply,
                balances,
                ..Default::default()
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).unwrap_or_default()
        }

        #[ink(message)]
        pub fn allowance_of(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();
            self.transfer_helper(&sender, &to, value)
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let allowance = self
                .allowances
                .get(&(from, self.env().caller()))
                .unwrap_or_default();

            if allowance < value {
                return Err(Error::AllowanceTooLow);
            }

            self.allowances
                .insert(&(from, self.env().caller()), &(allowance - value));

            self.transfer_helper(&from, &to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let sender = self.env().caller();

            self.allowances.insert(&(sender, to), &value);

            self.env().emit_event(Approval {
                from: sender,
                to,
                value,
            });

            Ok(())
        }

        pub fn transfer_helper(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let balance_from = self.balance_of(*from);
            let balance_to = self.balance_of(*to);

            if value > balance_from {
                return Err(Error::BalanceTooLow);
            }

            self.balances.insert(from, &(balance_from - value));
            self.balances.insert(to, &(balance_to + value));

            self.env().emit_event(Transfer {
                from: *from,
                to: *to,
                value,
            });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        type Event = <Erc20 as ::ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn default_works() {
            let erc20 = Erc20::default();
            assert_eq!(erc20.total_supply, 0);
        }

        #[ink::test]
        fn transfer_should_works() {
            let mut erc20 = Erc20::new(100000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(erc20.total_supply(), 100000);

            let val = 12345u128;

            let _ = erc20.transfer(accounts.bob, val);
            let emitted_evnets = ink::env::test::recorded_events().collect::<Vec<_>>();
            let event = &emitted_evnets[0];
            let decoded =
                <Event as scale::Decode>::decode(&mut &event.data[..]).expect("decoded error");
            match decoded {
                Event::Transfer(Transfer {
                    from: _,
                    to: _,
                    value,
                }) => {
                    assert_eq!(value, val);
                }
                _ => panic!("match error"),
            }
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            let mut erc20 = Erc20::new(100000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let res = erc20.transfer(accounts.charlie, 12);
            assert!(res.is_err());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;

        use ink_e2e::{
            build_message,
            //tokio::time::{sleep, Duration},
        };

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_transfer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let total_supply = 100000u128;
            let val = 2000u128;
            let constructor = Erc20Ref::new(total_supply);

            let contract_account = client
                .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed");
            let contract_account_id = contract_account.account_id;

            let bob_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let alice_acc = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);

            let msg1 = build_message::<Erc20Ref>(contract_account_id.clone())
                .call(|erc20| erc20.transfer(bob_acc.clone(), val));
            let res1 = client.call_dry_run(&ink_e2e::alice(), &msg1, 0, None).await;
            assert!(res1.exec_result.result.is_ok());

            let msg2 = build_message::<Erc20Ref>(contract_account_id.clone())
                .call(|erc20| erc20.balance_of(bob_acc));
            let res2 = client
                .call_dry_run(&ink_e2e::bob(), &msg2, 0, None)
                .await
                .return_value();
            assert_eq!(res2, val, "balance_of bob");

            let msg3 = build_message::<Erc20Ref>(contract_account_id.clone())
                .call(|erc20| erc20.balance_of(alice_acc));
            let res3 = client
                .call_dry_run(&ink_e2e::alice(), &msg3, 0, None)
                .await
                .return_value();
            assert_eq!(res3, total_supply - val, "balance_of alice");

            Ok(())
        }
    }
}
