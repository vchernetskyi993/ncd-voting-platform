import { connect, Contract, KeyPair } from "near-api-js";
import { InMemoryKeyStore } from "near-api-js/lib/key_stores";
import { NearConfig } from "../config";

type ElectionData = {
  start: bigint;
  end: bigint;
  title: string;
  description: string;
  candidates: string[];
};

type Candidate = {
  name: string;
  votes: string;
};

type ElectionView = {
  start: string;
  end: string;
  title: string;
  description: string;
  candidates: Candidate[];
};

type Page<T> = {
  pageNumber: number;
  pageSize: number;
  values: T[];
  elementsCount: bigint;
  pageCount: bigint;
};

export interface ElectionsContract {
  createElection(election: ElectionData): bigint;

  getElection(electionId: bigint): Promise<ElectionView>;

  getElections(pageNumber: number, pageSize: number): Page<ElectionView>;
}

export async function electionsContract(
  config: NearConfig
): Promise<ElectionsContract> {
  const contract = await contractProxy(config);

  return {
    createElection(election: ElectionData): bigint {
      return BigInt(0);
    },
    getElection(electionId: bigint): Promise<ElectionView> {
      return contract.get_election({
        organization_id: config.account,
        election_id: electionId.toString(),
      });
    },
    getElections(pageNumber: number, pageSize: number): Page<ElectionView> {
      return {
        pageNumber,
        pageSize: 0,
        values: [],
        elementsCount: BigInt(0),
        pageCount: BigInt(0),
      };
    },
  };
}

interface ContractProxy {
  create_election(args: { election: ElectionData }): Promise<string>;

  elections_count(args: { organization_id: string }): Promise<string>;

  get_election(args: {
    organization_id: string;
    election_id: string;
  }): Promise<ElectionView>;
}

async function contractProxy(config: NearConfig): Promise<ContractProxy> {
  const keyStore = new InMemoryKeyStore();
  const keyPair = KeyPair.fromString(config.privateKey);
  await keyStore.setKey(config.networkId, config.account, keyPair);
  const near = await connect({
    networkId: config.networkId,
    keyStore,
    nodeUrl: config.nodeUrl,
    headers: {},
  });
  const account = await near.account(config.account);
  return new Contract(account, config.contract, {
    viewMethods: ["elections_count", "get_election"],
    changeMethods: ["create_election"],
  }) as any as ContractProxy;
}
