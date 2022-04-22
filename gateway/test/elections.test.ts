import { Account, connect, Contract, Near } from "near-api-js";
import {
  KeyStore,
  UnencryptedFileSystemKeyStore,
} from "near-api-js/lib/key_stores";
import fs from "fs";
import { KeyPairEd25519, PublicKey } from "near-api-js/lib/utils";
import net, { AddressInfo } from "net";
import { startServer } from "../src/server";
import { ApplicationConfig } from "../src/config";
import { Server } from "http";
import axios from "axios";
import { expect } from "chai";
import dayjs from "dayjs";

const TEST_NETWORK = "shared-test";
const TEST_NODE_URL = "https://rpc.ci-testnet.near.org";
const TEST_ACCOUNT = "test.near";

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

interface ElectionsContract {
  "new"(args: {}): Promise<void>;

  register_organization(args: { account: string }): Promise<void>;

  create_election(
    args: {
      input: ElectionData;
    },
    gas: string,
    amount: string
  ): Promise<string>;
}

describe("Election API tests", () => {
  let port: number;
  let server: Server;
  let contract: ElectionsContract;

  before(async () => {
    const keyStore = new UnencryptedFileSystemKeyStore("neardev");
    const near = await connectToNear(keyStore);
    const contractName = `test-account-${Date.now()}-${Math.floor(
      100000 + Math.random() * 900000
    )}`;
    console.log(`Deploying contract ${contractName}...`);
    const contractAccount = await deployContract(keyStore, near, contractName);
    const ownedContract = createContract(contractAccount, contractName);
    console.log(`Initializing contract...`);
    await ownedContract.new({});

    const organizationName = `test-org.${contractName}`;
    console.log(`Creating and registering organization...`);
    const organizationKeyPair = KeyPairEd25519.fromRandom();
    await Promise.all([
      createOrganization(
        organizationName,
        contractAccount,
        organizationKeyPair.getPublicKey()
      ),
      ownedContract.register_organization({ account: contractName }),
      ownedContract.register_organization({ account: organizationName }),
    ]);

    await keyStore.setKey(TEST_NETWORK, organizationName, organizationKeyPair);
    const organization = await near.account(organizationName);
    contract = createContract(organization, contractName);

    console.log(`Starting server...`);
    port = await freePort();
    server = await startServer(
      populateConfig(
        port,
        organizationName,
        organizationKeyPair.secretKey,
        contractName
      )
    );
  });

  it("Should get election", async () => {
    // given
    const saved = election();
    const id = await contract.create_election(
      {
        input: saved,
      },
      "300000000000000",
      (10n ** 24n).toString()
    );

    // when
    const response = await axios.get(
      `http://localhost:${port}/elections/${id}`
    );

    // then
    expect(response.status).to.equal(200);
    const fetched = response.data as ElectionView;
    expect(fetched.start).to.equal(saved.start);
    expect(fetched.end).to.equal(saved.end);
    expect(fetched.title).to.equal(saved.title);
    expect(fetched.description).to.equal(saved.description);
    saved.candidates.forEach((name, i) => {
      const candidate = fetched.candidates[i];
      expect(candidate.name).to.equal(name);
      expect(candidate.votes).to.equal("0");
    });
  });

  after(async () => {
    await new Promise<void>((resolve) => server.close(() => resolve()));
  });
});

async function connectToNear(keyStore: KeyStore): Promise<Near> {
  return await connect({
    networkId: TEST_NETWORK,
    nodeUrl: TEST_NODE_URL,
    masterAccount: TEST_ACCOUNT,
    headers: {},
    keyStore,
  });
}

async function deployContract(
  keyStore: KeyStore,
  near: Near,
  contractName: string
): Promise<Account> {
  const testAccount = await near.account(TEST_ACCOUNT);
  const keyPair = KeyPairEd25519.fromRandom();
  const binary = new Uint8Array(
    fs.readFileSync("../contract/res/elections.wasm")
  );
  await keyStore.setKey(TEST_NETWORK, contractName, keyPair);
  return await testAccount.createAndDeployContract(
    contractName,
    keyPair.getPublicKey(),
    binary,
    "500000000000000000000000000"
  );
}

function createContract(account: Account, name: string): ElectionsContract {
  return new Contract(account, name, {
    viewMethods: [],
    changeMethods: ["new", "register_organization", "create_election"],
  }) as any as ElectionsContract;
}

async function createOrganization(
  name: string,
  contractAccount: Account,
  publicKey: PublicKey
): Promise<void> {
  await contractAccount.createAccount(
    name,
    publicKey,
    "100000000000000000000000000"
  );
}

function freePort(): Promise<number> {
  const server = net.createServer();
  return new Promise<number>((resolve) =>
    server.listen(0, () => {
      resolve((server.address() as AddressInfo).port);
    })
  ).finally(() => server.close());
}

function populateConfig(
  port: number,
  account: string,
  privateKey: string,
  contract: string
): ApplicationConfig {
  return {
    serverPort: port,
    near: {
      networkId: TEST_NETWORK,
      nodeUrl: TEST_NODE_URL,
      account,
      privateKey,
      contract,
    },
  };
}

function election(): ElectionData {
  return {
    start: dayjs().add(1, "day").unix() + "000000000",
    end: dayjs().add(3, "day").unix() + "000000000",
    title: "Important election",
    description: "Important description",
    candidates: ["valuable choice", "even more valuable choice"],
  };
}
