import dotenv from "dotenv";

export type NearConfig = {
  networkId: string;
  nodeUrl: string;
  account: string;
  privateKey: string;
  contract: string;
};

export type ApplicationConfig = {
  serverPort: number;
  near: NearConfig;
};

export function initializeConfig(): ApplicationConfig {
  dotenv.config();
  return {
    serverPort: +process.env.SERVER_PORT,
    near: {
      networkId: process.env.NEAR_NETWORK_ID,
      nodeUrl: process.env.NEAR_NODE_URL,
      account: process.env.NEAR_ACCOUNT,
      privateKey: process.env.NEAR_PRIVATE_KEY,
      contract: process.env.NEAR_CONTRACT,
    },
  };
}
