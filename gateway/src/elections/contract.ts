import { connect, Contract, KeyPair } from "near-api-js";
import { InMemoryKeyStore } from "near-api-js/lib/key_stores";
import { NearConfig } from "../config";

type ElectionData = {
  start: string;
  end: string;
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
  pageNumber: string;
  pageSize: number;
  values: T[];
  elementsCount: string;
  pageCount: string;
};

export interface ElectionsContract {
  createElection(election: ElectionData): Promise<string>;

  getElection(electionId: string): Promise<ElectionView>;

  getElections(
    pageNumber: string,
    pageSize: string
  ): Promise<Page<ElectionView>>;
}

export async function electionsContract(
  config: NearConfig
): Promise<ElectionsContract> {
  const contract = await contractProxy(config);

  return {
    createElection(election: ElectionData): Promise<string> {
      return contract.create_election(
        { input: election },
        undefined,
        "1000000000000000000000000"
      );
    },
    getElection(electionId: string): Promise<ElectionView> {
      return contract.get_election({
        organization_id: config.account,
        election_id: electionId,
      });
    },
    async getElections(
      pageNumber: string,
      pageSize: string
    ): Promise<Page<ElectionView>> {
      const count = await contract.elections_count({
        organization_id: config.account,
      });
      const start = BigInt(pageSize) * (BigInt(pageNumber) - 1n);
      const length =
        pageCount(count, pageSize).toString() !== pageNumber
          ? +pageSize
          : Number(BigInt(count) % BigInt(pageSize));
      const elections = await Promise.all(
        Array.from(new Array(length).keys(), (i) => start + BigInt(i))
          .map((i) => i.toString())
          .map((i) =>
            contract.get_election({
              organization_id: config.account,
              election_id: i,
            })
          )
      );
      return {
        pageNumber,
        pageSize: elections.length,
        values: elections,
        elementsCount: count,
        pageCount: pageCount(count, pageSize).toString(),
      };
    },
  };
}

function pageCount(elementsCount: string, pageSize: string): bigint {
  return (
    BigInt(elementsCount) / BigInt(pageSize) +
    (BigInt(elementsCount) % BigInt(pageSize) !== 0n ? 1n : 0n)
  );
}

interface ContractProxy {
  create_election(
    args: { input: ElectionData },
    gas: undefined,
    amount: "1000000000000000000000000"
  ): Promise<string>;

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
