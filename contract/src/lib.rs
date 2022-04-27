use std::convert::TryInto;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

const CREATE_ELECTION_COST: u128 = 1; // NEAR

const NOT_REGISTERED_ERROR: &str = "Account is not registered as a valid organization.";
const CANDIDATES_LIMIT: u16 = 256;

/// Contract for performing public elections between values.
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

/// Election data actually stored.
#[derive(BorshDeserialize, BorshSerialize)]
struct Election {
    start: u64,
    end: u64,
    title: String,
    description: String,
    candidates: Vec<String>,
}

impl Election {
    fn new(input: &ElectionInput) -> Self {
        Self {
            start: input.start.parse().unwrap(),
            end: input.end.parse().unwrap(),
            title: input.title.clone(),
            description: input.description.clone(),
            candidates: input.candidates.clone(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ElectionInput {
    start: String,
    end: String,
    title: String,
    description: String,
    candidates: Vec<String>,
}

/// Election view for clients.
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ElectionView {
    start: String,
    end: String,
    title: String,
    description: String,
    candidates: Vec<Candidate>,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
struct Candidate {
    name: String,
    votes: String,
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
    /// Contract init function. Could be called only once.
    ///
    /// Sets the caller as contract owner.
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

    /// Register account as an organization.
    ///
    /// # Arguments
    ///
    /// * `account` - [AccountId](../near_sdk/struct.AccountId.html) of an organization
    ///
    /// # Panics
    ///
    /// * Only owner is allowed to call this function.
    pub fn register_organization(&mut self, account: &OrganizationId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can register new organizations"
        );
        self.organizations.insert(account, &0);
    }

    /// Create new election.
    ///
    /// # Arguments
    ///
    /// * `election` - initial [Election](struct.Election.html) data to store
    ///
    /// # Panics
    ///
    /// * Function is a paid one. Expects exactly **1 NEAR** deposit.
    /// * Only registered organization is allowed to call this function.
    /// * Candidates array length should be between 2 and 256 elements.
    /// * Start and end dates are validated based on block timestamp.
    ///   They both should be in the future and end should be after start.
    #[payable]
    pub fn create_election(&mut self, input: &ElectionInput) -> String {
        let election = Election::new(input);
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
        self.elections.insert(&(organization_id, id), &election);
        id.to_string()
    }

    /// Returns number of elections for an organization.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - [AccountId](../near_sdk/struct.AccountId.html) of an organization
    ///
    /// # Panics
    ///
    /// * Organization should be registered.
    pub fn elections_count(&self, organization_id: &OrganizationId) -> String {
        self.organizations
            .get(organization_id)
            .expect(NOT_REGISTERED_ERROR)
            .to_string()
    }

    /// Returns election data.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - [AccountId](../near_sdk/struct.AccountId.html) of an organization
    /// * `election_id` - String id
    ///
    /// # Panics
    ///
    /// * `election_id` can not be parsed as u128
    /// * Election not found.
    pub fn get_election(
        &self,
        organization_id: &OrganizationId,
        election_id: &String,
    ) -> ElectionView {
        let election = self
            .elections
            .get(&(
                organization_id.clone(),
                election_id.parse::<u128>().unwrap(),
            ))
            .expect("Election not found");

        ElectionView {
            start: election.start.to_string(),
            end: election.end.to_string(),
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
                            election_id.parse().unwrap(),
                            i.try_into().unwrap(),
                        ))
                        .unwrap_or(0)
                        .to_string(),
                })
                .collect(),
        }
    }

    /// Checks if caller has already voted.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - [AccountId](../near_sdk/struct.AccountId.html) of an organization
    /// * `election_id` - String id
    ///
    /// # Panics
    ///
    /// * `election_id` can not be parsed as u128
    pub fn have_voted(&mut self, organization_id: &OrganizationId, election_id: &String) -> bool {
        self.voters.contains(&(
            organization_id.clone(),
            election_id.parse().unwrap(),
            env::predecessor_account_id(),
        ))
    }

    /// Vote in some election.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - [AccountId](../near_sdk/struct.AccountId.html) of an organization
    /// * `election_id` - String id
    /// * `candidate_id` - u8 id
    ///
    /// # Panics
    ///
    /// * `election_id` should be parsed as u128.
    /// * `organization_id` & `election_id` & `candidate_id` should be a valid combination.
    /// * Current date should be between start and end dates of the election.
    /// * User shouldn't try to vote more than once.
    pub fn vote(
        &mut self,
        organization_id: &OrganizationId,
        election_id: &String,
        candidate_id: u8,
    ) {
        let election_id_parsed = election_id.parse().unwrap();
        let election = self
            .elections
            .get(&(organization_id.clone(), election_id_parsed))
            .unwrap();
        assert!(
            election.start < env::block_timestamp(),
            "Election not started yet"
        );
        assert!(
            election.end > env::block_timestamp(),
            "Election already ended"
        );
        let voter_key = &(
            organization_id.clone(),
            election_id_parsed,
            env::predecessor_account_id(),
        );
        assert!(!self.voters.contains(voter_key), "User already voted");

        let candidate_key = &(organization_id.clone(), election_id_parsed, candidate_id);
        let votes = self.votes.get(candidate_key).unwrap_or(0);
        self.votes.insert(candidate_key, &(votes + 1));
        self.voters.insert(voter_key);
    }
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
        let input = ElectionInput::new();

        let id = contract.create_election(&input).parse().unwrap();

        assert_eq!(id, 0);
        assert_eq!(contract.organizations.get(&organization).unwrap(), 1);
        assert!(contract.elections.contains_key(&(organization.clone(), id)));
        let saved = contract.elections.get(&(organization, id)).unwrap();
        assert_eq!(saved.start.to_string(), input.start);
        assert_eq!(saved.end.to_string(), input.end);
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

        contract.create_election(&ElectionInput::new());
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
        let input = ElectionInput::new()
            .set_start(Utc::now().checked_sub_signed(Duration::days(1)).unwrap());

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
        let input = ElectionInput::new().set_end(Utc::now());

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
        let input = ElectionInput::new().set_candidates(vec![]);

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
        let input = ElectionInput::new().set_candidates(vec!["Alice".to_string()]);

        contract.create_election(&input);
    }

    #[test]
    #[should_panic(expected = "paid")]
    fn should_require_deposit_on_create() {
        let mut contract = create_contract();
        contract.organizations.insert(&account(ORGANIZATION), &0);
        prepare_env(ORGANIZATION);

        contract.create_election(&ElectionInput::new());
    }

    #[test]
    #[should_panic(expected = "paid")]
    fn should_require_exact_deposit_on_create() {
        let mut contract = create_contract();
        contract.organizations.insert(&account(ORGANIZATION), &0);
        testing_env!(context(ORGANIZATION)
            .attached_deposit(EXPECTED_CREATE_ELECTION_COST * 2)
            .build());

        contract.create_election(&ElectionInput::new());
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
            &ElectionInput::new()
                .set_candidates((0..).take(257).map(|_| "Bob".to_string()).collect()),
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

        assert_eq!(result, count.to_string());
    }

    #[test]
    fn should_return_true_if_voted() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        let user = account(USER);
        let election_id = 12;
        contract
            .voters
            .insert(&(organization.clone(), election_id, user));
        prepare_env(USER);

        let result = contract.have_voted(&organization, &election_id.to_string());

        assert!(result);
    }

    #[test]
    fn should_return_false_if_not_voted() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        let election_id = 12;
        prepare_env(USER);

        let result = contract.have_voted(&organization, &election_id.to_string());

        assert!(!result);
    }

    #[test]
    fn should_fetch_election() {
        let organization = account(ORGANIZATION);
        let mut contract = create_contract();
        let election_id = 1;
        let input = ElectionInput::new();
        contract
            .elections
            .insert(&(organization.clone(), election_id), &Election::new(&input));
        let bob_votes = 3;
        contract
            .votes
            .insert(&(organization.clone(), election_id, 1), &bob_votes);
        prepare_env(USER);

        let result = contract.get_election(&organization, &election_id.to_string());

        assert_eq!(result.start, input.start);
        assert_eq!(result.end, input.end);
        assert_eq!(result.title, input.title);
        assert_eq!(result.description, input.description);
        assert_eq!(result.candidates.len(), 2);
        let alice = result.candidates.get(0).unwrap();
        assert_eq!(alice.name, "Alice".to_string());
        assert_eq!(alice.votes, "0");
        let bob = result.candidates.get(1).unwrap();
        assert_eq!(bob.name, "Bob".to_string());
        assert_eq!(bob.votes, bob_votes.to_string());
    }

    #[test]
    #[should_panic(expected = "started")]
    fn should_panic_if_election_not_started() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        let election_id = 1;
        contract.elections.insert(
            &(organization.clone(), election_id),
            &Election::new(&ElectionInput::new()),
        );
        prepare_env(USER);

        contract.vote(&organization, &election_id.to_string(), 0);
    }

    #[test]
    #[should_panic(expected = "ended")]
    fn should_panic_if_election_finished() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        let election_id = 1;
        contract.elections.insert(
            &(organization.clone(), election_id),
            &Election::new(&ElectionInput::new()),
        );
        testing_env!(context(USER)
            .block_timestamp(nanoseconds(
                Utc::now().checked_add_signed(Duration::days(4)).unwrap()
            ))
            .build());

        contract.vote(&organization, &election_id.to_string(), 0);
    }

    #[test]
    fn should_vote() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        let election_id = 1;
        contract.elections.insert(
            &(organization.clone(), election_id),
            &Election::new(&ElectionInput::new()),
        );
        testing_env!(context(USER)
            .block_timestamp(nanoseconds(
                Utc::now().checked_add_signed(Duration::days(2)).unwrap()
            ))
            .build());
        let candidate_id = 1;

        contract.vote(&organization, &election_id.to_string(), candidate_id);

        assert_eq!(
            contract
                .votes
                .get(&(organization.clone(), election_id, candidate_id))
                .unwrap(),
            1
        );
        assert!(contract
            .voters
            .contains(&(organization.clone(), election_id, account(USER))));
    }

    #[test]
    #[should_panic(expected = "already voted")]
    fn should_prohibit_to_vote_twice() {
        let mut contract = create_contract();
        let organization = account(ORGANIZATION);
        let election_id = 1;
        contract.elections.insert(
            &(organization.clone(), election_id),
            &Election::new(&ElectionInput::new()),
        );
        testing_env!(context(USER)
            .block_timestamp(nanoseconds(
                Utc::now().checked_add_signed(Duration::days(2)).unwrap()
            ))
            .build());
        let candidate_id = 1;

        contract.vote(&organization, &election_id.to_string(), candidate_id);
        contract.vote(&organization, &election_id.to_string(), candidate_id);
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

    impl ElectionInput {
        fn new() -> Self {
            Self {
                start: nanoseconds(Utc::now().checked_add_signed(Duration::days(1)).unwrap())
                    .to_string(),
                end: nanoseconds(Utc::now().checked_add_signed(Duration::days(3)).unwrap())
                    .to_string(),
                title: "My Election".to_string(),
                description: "My Description".to_string(),
                candidates: vec!["Alice".to_string(), "Bob".to_string()],
            }
        }

        fn set_start(mut self, start: DateTime<Utc>) -> Self {
            self.start = nanoseconds(start).to_string();
            self
        }

        fn set_end(mut self, end: DateTime<Utc>) -> Self {
            self.end = nanoseconds(end).to_string();
            self
        }

        fn set_candidates(mut self, candidates: Vec<String>) -> Self {
            self.candidates = candidates;
            self
        }
    }
}
