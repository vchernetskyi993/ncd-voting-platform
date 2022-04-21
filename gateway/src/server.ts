import { Server } from "http";
import express from "express";
import { electionsRouter } from "./elections/routes";

export function startServer(): Server {
  const app = express();

  app.use("/elections", electionsRouter());

  const port = process.env.SERVER_PORT;
  return app.listen(port, () => {
    console.log(`Express is listening on port ${port}`);
  });
}
