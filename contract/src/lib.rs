use std::convert::TryInto;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

const CREATE_ELECTION_COST: u128 = 1; // NEAR

const NOT_REGISTERED_ERROR: &str = "Account is not registered as a valid organization.";
const CANDIDATES_LIMIT: u16 = 256;

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Elections {
    owner_id: AccountId,
    organizations: LookupMap<OrganizationId, ElectionCount>,
    elections: LookupMap<(OrganizationId, ElectionId), Election>,
    votes: LookupMap<(OrganizationId, ElectionId, CandidateId), u128>,
    voters: LookupSet<(OrganizationId, ElectionId, VoterId)>,
}

type OrganizationId = AccountId;
type ElectionCount = u128;
type ElectionId = u128;
type CandidateId = u8;
type VoterId = AccountId;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Election {
    start: u64,
    end: u64,
    title: String,
    description: String,
    candidates: Vec<String>,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ElectionView {
    start: u64,
    end: u64,
    title: String,
    description: String,
    candidates: Vec<Candidate>,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
struct Candidate {
    name: String,
    votes: u128,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKeys {
    Organizations,
    Elections,
    Results,
    Voters,
}

#[near_bindgen]
impl Elections {
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            organizations: LookupMap::new(StorageKeys::Organizations),
            elections: LookupMap::new(StorageKeys::Elections),
            votes: LookupMap::new(StorageKeys::Results),
            voters: LookupSet::new(StorageKeys::Voters),
        }
    }

    pub fn register_organization(&mut self, account: &OrganizationId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can register new organizations"
        );
        self.organizations.insert(account, &0);
    }

    #[payable]
    pub fn create_election(&mut self, election: &Election) -> u128 {
        assert!(
            election.candidates.len() > 1,
            "More than one candidate should be provided"
        );
        assert!(
            election.candidates.len() <= CANDIDATES_LIMIT.into(),
            "Maximum {} candidates expected",
            CANDIDATES_LIMIT,
        );
        assert!(
            election.start > env::block_timestamp(),
            "Start should be in the future"
        );
        assert!(election.start < election.end, "Start should be before end");
        assert!(
            env::attached_deposit() == to_yocto(CREATE_ELECTION_COST),
            "Create election is paid function. Expects to receive exactly {} NEAR",
            CREATE_ELECTION_COST
        );

        let organization_id = env::predecessor_account_id();
        let id = self
            .organizations
            .get(&organization_id)
            .expect(NOT_REGISTERED_ERROR);
        self.organizations.insert(&organization_id, &(id + 1));
        self.elections.insert(&(organization_id, id), election);
        id
    }

    pub fn elections_count(&self, organization_id: &OrganizationId) -> u128 {
        self.organizations
            .get(organization_id)
            .expect(NOT_REGISTERED_ERROR)
    }

    pub fn get_election(
        &self,
        organization_id: &OrganizationId,
        election_id: &ElectionId,
    ) -> ElectionView {
        let election = self
            .elections
            .get(&(organization_id.clone(), election_id.clone()))
            .expect("Election not found");

        ElectionView {
            start: election.start,
            end: election.end,
            title: election.title,
            description: election.description,
            candidates: election
                .candidates
                .iter()
                .enumerate()
                .map(|(i, candidate)| Candidate {
                    name: candidate.clone(),
                    votes: self
                        .votes
                        .get(&(
                            organization_id.clone(),
                            election_id.clone(),
                            i.try_into().expect(&format!(
                                "Maximum {} candidates expected",
                                CANDIDATES_LIMIT
                            )),
                        ))
                        .unwrap_or(0),
                })
                .collect(),
        }
    }

    // TODO: implement voter functions
    pub fn have_voted() {}

    pub fn vote() {}
}

fn to_yocto(n: u128) -> u128 {
    n * (10 as u128).pow(24)
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;
    use chrono::{DateTime, Duration, Utc};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    const OWNER: &str = "alice.testnet";
    const USER: &str = "bob.testnet";
    const ORGANIZATION: &str = "org1.testnet";
    const EXPECTED_CREATE_ELECTION_COST: u128 = 1_000_000_000_000_000_000_000_000;

    #[test]
    fn should_create_organization() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);

        contract.register_organization(&organization);

        assert!(contract.organizations.contains_key(&organization));
        assert_eq!(contract.organizations.get(&organization).unwrap(), 0);
    }

    #[test]
    #[should_panic(expected = "Only owner")]
    fn should_not_create_organization_by_non_owner() {
        let mut contract = create_contract();
        prepare_env(USER);
        let organization = account(ORGANIZATION);

        contract.register_organization(&organization);
    }

    #[test]
    fn should_create_election() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        contract.organizations.insert(&organization, &0);
        testing_env!(context(ORGANIZATION)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST)
            .build());
        let input = Election::new();

        let id = contract.create_election(&input);

        assert_eq!(id, 0);
        assert_eq!(contract.organizations.get(&organization).unwrap(), 1);
        assert!(contract.elections.contains_key(&(organization.clone(), id)));
        let saved = contract.elections.get(&(organization, id)).unwrap();
        assert_eq!(saved.start, input.start);
        assert_eq!(saved.end, input.end);
        assert_eq!(saved.title, input.title);
        assert_eq!(saved.description, input.description);
        assert_eq!(saved.candidates, input.candidates);
    }

    #[test]
    #[should_panic(expected = "not registered")]
    fn should_check_organization_registration_on_create() {
        let mut contract = create_contract();
        contract.organizations.insert(&account(ORGANIZATION), &0);
        testing_env!(context(USER)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST)
            .build());

        contract.create_election(&Election::new());
    }

    #[test]
    #[should_panic(expected = "in the future")]
    fn should_check_election_start_date_on_create() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        contract.organizations.insert(&organization, &0);
        testing_env!(context(ORGANIZATION)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST)
            .build());
        let input =
            Election::new().set_start(Utc::now().checked_sub_signed(Duration::days(1)).unwrap());

        contract.create_election(&input);
    }

    #[test]
    #[should_panic(expected = "before end")]
    fn should_check_election_end_date_on_create() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        contract.organizations.insert(&organization, &0);
        testing_env!(context(ORGANIZATION)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST)
            .build());
        let input = Election::new().set_end(Utc::now());

        contract.create_election(&input);
    }

    #[test]
    #[should_panic(expected = "provided")]
    fn should_check_that_candidates_are_provided() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        contract.organizations.insert(&organization, &0);
        testing_env!(context(ORGANIZATION)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST)
            .build());
        let input = Election::new().set_candidates(vec![]);

        contract.create_election(&input);
    }

    #[test]
    #[should_panic(expected = "provided")]
    fn should_check_that_more_than_one_candidate_provided() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        contract.organizations.insert(&organization, &0);
        testing_env!(context(ORGANIZATION)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST)
            .build());
        let input = Election::new().set_candidates(vec!["Alice".to_string()]);

        contract.create_election(&input);
    }

    #[test]
    #[should_panic(expected = "paid")]
    fn should_require_deposit_on_create() {
        let mut contract = create_contract();
        contract.organizations.insert(&account(ORGANIZATION), &0);
        prepare_env(ORGANIZATION);

        contract.create_election(&Election::new());
    }

    #[test]
    #[should_panic(expected = "paid")]
    fn should_require_exact_deposit_on_create() {
        let mut contract = create_contract();
        contract.organizations.insert(&account(ORGANIZATION), &0);
        testing_env!(context(ORGANIZATION)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST * 2)
            .build());

        contract.create_election(&Election::new());
    }

    #[test]
    #[should_panic(expected = "256")]
    fn should_allow_maximum_256_candidates() {
        let mut contract = create_contract();
        contract.organizations.insert(&account(ORGANIZATION), &0);
        testing_env!(context(ORGANIZATION)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST)
            .build());

        contract.create_election(
            &Election::new().set_candidates((0..).take(257).map(|_| "Bob".to_string()).collect()),
        );
    }

    #[test]
    fn should_fetch_election_count() {
        let count = 14;
        let organization = account(ORGANIZATION);
        let mut contract = create_contract();
        contract.organizations.insert(&organization, &count);
        prepare_env(USER);

        let result = contract.elections_count(&organization);

        assert_eq!(result, count);
    }

    #[test]
    fn should_fetch_election() {
        let organization = account(ORGANIZATION);
        let mut contract = create_contract();
        let election_id = 1;
        let saved = Election::new();
        contract
            .elections
            .insert(&(organization.clone(), election_id), &saved);
        let bob_votes = 3;
        contract
            .votes
            .insert(&(organization.clone(), election_id, 1), &bob_votes);
        prepare_env(USER);

        let result = contract.get_election(&organization, &election_id);

        assert_eq!(result.start, saved.start);
        assert_eq!(result.end, saved.end);
        assert_eq!(result.title, saved.title);
        assert_eq!(result.description, saved.description);
        assert_eq!(result.candidates.len(), 2);
        let alice = result.candidates.get(0).unwrap();
        assert_eq!(alice.name, "Alice".to_string());
        assert_eq!(alice.votes, 0);
        let bob = result.candidates.get(1).unwrap();
        assert_eq!(bob.name, "Bob".to_string());
        assert_eq!(bob.votes, bob_votes);
    }

    fn create_contract() -> Elections {
        prepare_env(OWNER);
        Elections::new()
    }

    fn prepare_env(predecessor: &str) {
        testing_env!(context(predecessor).build())
    }

    fn context(predecessor: &str) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(account(predecessor))
            .block_timestamp(nanoseconds(Utc::now()));
        builder
    }

    fn account(name: &str) -> AccountId {
        AccountId::new_unchecked(name.to_string())
    }

    fn nanoseconds(date: DateTime<Utc>) -> u64 {
        date.timestamp_nanos().try_into().unwrap()
    }

    impl Election {
        fn new() -> Self {
            Self {
                start: nanoseconds(Utc::now().checked_add_signed(Duration::days(1)).unwrap()),
                end: nanoseconds(Utc::now().checked_add_signed(Duration::days(3)).unwrap()),
                title: "My Election".to_string(),
                description: "My Description".to_string(),
                candidates: vec!["Alice".to_string(), "Bob".to_string()],
            }
        }

        fn set_start(mut self, start: DateTime<Utc>) -> Self {
            self.start = nanoseconds(start);
            self
        }

        fn set_end(mut self, end: DateTime<Utc>) -> Self {
            self.end = nanoseconds(end);
            self
        }

        fn set_candidates(mut self, candidates: Vec<String>) -> Self {
            self.candidates = candidates;
            self
        }
    }
}
