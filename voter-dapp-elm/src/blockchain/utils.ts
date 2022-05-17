import { connect, WalletConnection } from "near-api-js";
import { BrowserLocalStorageKeyStore } from "near-api-js/lib/key_stores";
import { NearConfig } from "near-api-js/lib/near";

export async function getWallet(): Promise<WalletConnection> {
  const envId = process.env.NEAR_ENV!;
  const config = getConfig(envId);
  const near = await connect(config);
  return new WalletConnection(near, null);
}

function getConfig(envId: string): NearConfig {
  switch (envId) {
    case "testnet":
      return {
        networkId: "testnet",
        keyStore: new BrowserLocalStorageKeyStore(),
        nodeUrl: "https://rpc.testnet.near.org",
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        headers: {},
      };
    default:
      throw Error(`Unsupported NEAR env '${envId}'`);
  }
}
