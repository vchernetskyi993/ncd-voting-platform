use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet, Vector};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Elections {
    owner_id: AccountId,
    organizations: UnorderedSet<AccountId>,
    elections: LookupMap<AccountId, Vector<Election>>,
}

struct Election {}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKeys {
    Organizations,
    AllElections,
    OrganizationElections,
}

#[near_bindgen]
impl Elections {
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            organizations: UnorderedSet::new(StorageKeys::Organizations),
            elections: LookupMap::new(StorageKeys::AllElections),
        }
    }

    pub fn register_organization(&mut self, account: &AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can register new organizations"
        );
        self.organizations.insert(account);
        self.elections
            .insert(account, &Vector::new(StorageKeys::OrganizationElections));
    }

    // organization only
    pub fn create_election() {}

    // any user
    pub fn elections_count() {}

    pub fn get_election() {}

    pub fn have_voted() {}

    pub fn vote() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    #[test]
    fn should_create_organization() {
        prepare_env("alice.testnet");
        let mut contract = Elections::new();
        let organization = account("org1.testnet");

        contract.register_organization(&organization);

        assert!(contract.organizations.contains(&organization));
        assert!(contract.elections.contains_key(&organization));
    }

    #[test]
    #[should_panic(expected = "Only owner")]
    fn should_not_create_organization_by_non_owner() {
        prepare_env("alice.testnet");
        let mut contract = Elections::new();
        prepare_env("bob.testnet");
        let organization = account("org1.testnet");

        contract.register_organization(&organization);
    }

    fn prepare_env(predecessor: &str) {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(account(predecessor))
            .build())
    }

    fn account(name: &str) -> AccountId {
        AccountId::new_unchecked(name.to_string())
    }
}
