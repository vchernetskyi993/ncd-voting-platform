import { Server } from "http";
import express from "express";
import { electionsRouter } from "./elections/routes";
import { ApplicationConfig } from "./config";
import { electionsContract } from "./elections/contract";

export async function startServer(config: ApplicationConfig): Promise<Server> {
  const app = express();

  app.use(express.json());
  app.set("json replacer", (key, value) =>
    typeof value === "bigint" ? value.toString() : value
  );
  app.use("/elections", electionsRouter(await electionsContract(config.near)));

  return app.listen(config.serverPort, () => {
    console.log(`Express is listening on port ${config.serverPort}`);
  });
}
