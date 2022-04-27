import { Account, Contract } from "near-api-js";

export function getElectionsContract(account: Account): Elections {
  const contract = new Contract(
    account,
    process.env.REACT_APP_ELECTIONS_CONTRACT_ID!,
    {
      viewMethods: ["elections_count", "get_election"],
      changeMethods: ["vote", "have_voted"],
    }
  ) as any as ElectionsContract;
  return new Elections(contract, process.env.REACT_APP_ORGANIZATION_ID!);
}

type Candidate = {
  name: string;
  votes: string;
};

export type Election = {
  start: string;
  end: string;
  title: string;
  description: string;
  candidates: Candidate[];
};

export class Elections {
  organizationId: string;
  contract: ElectionsContract;

  constructor(contract: ElectionsContract, organizationId: string) {
    this.contract = contract;
    this.organizationId = organizationId;
  }

  getElection(electionId: bigint): Promise<Election> {
    return this.contract.get_election({
      organization_id: this.organizationId,
      election_id: electionId.toString(),
    });
  }

  electionsCount(): Promise<bigint> {
    return this.contract
      .elections_count({
        organization_id: this.organizationId,
      })
      .then(BigInt);
  }

  vote(electionId: bigint, candidateId: number): Promise<void> {
    return this.contract.vote({
      organization_id: this.organizationId,
      election_id: electionId.toString(),
      candidate_id: candidateId,
    });
  }

  haveVoted(electionId: bigint): Promise<boolean> {
    return this.contract.have_voted({
      organization_id: this.organizationId,
      election_id: electionId.toString(),
    });
  }
}

interface ElectionsContract {
  elections_count(args: { organization_id: string }): Promise<string>;

  get_election(args: {
    organization_id: string;
    election_id: string;
  }): Promise<Election>;

  vote(args: {
    organization_id: string;
    election_id: string;
    candidate_id: number;
  }): Promise<void>;

  have_voted(args: {
    organization_id: string;
    election_id: string;
  }): Promise<boolean>;
}
