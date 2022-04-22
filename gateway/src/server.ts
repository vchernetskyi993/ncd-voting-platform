import { Server } from "http";
import express from "express";
import { electionsRouter } from "./elections/routes";
import { ApplicationConfig } from "./config";
import { electionsContract } from "./elections/contract";

export async function startServer(config: ApplicationConfig): Promise<Server> {
  const app = express();

  app.use(express.json());
  app.use("/elections", electionsRouter(await electionsContract(config.near)));

  return app.listen(config.serverPort, () => {
    console.log(`Express is listening on port ${config.serverPort}`);
  });
}
