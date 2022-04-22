import express, { Router } from "express";
import { ElectionsContract } from "./contract";

export function electionsRouter(contract: ElectionsContract): Router {
  return express
    .Router()
    .post("/", (req, res) => {
      res.status(201).json({ id: contract.createElection(req.body) });
    })
    .get("/:electionId", (req, res, next) => {
      contract
        .getElection(BigInt(req.params.electionId))
        .then((election) => res.json(election))
        .catch((error) => next(error));
    })
    .get("/", (req, res) => {
      const pageNumber = req.query.page || "1";
      const pageSize = req.query.pageSize || "10";
      res.json(contract.getElections(+pageNumber, +pageSize));
    });
}
