import express, { Router } from "express";
import { ElectionsContract } from "./contract";

export function electionsRouter(contract: ElectionsContract): Router {
  return express
    .Router()
    .post("/", (req, res, next) => {
      contract
        .createElection(req.body)
        .then((id) => res.status(201).json({ id }))
        .catch(next);
    })
    .get("/:electionId", (req, res, next) => {
      contract
        .getElection(req.params.electionId)
        .then((election) => res.json(election))
        .catch(next);
    })
    .get("/", (req, res, next) => {
      const pageNumber = req.query.page || "1";
      const pageSize = req.query.pageSize || "10";
      contract
        .getElections(pageNumber.toString(), pageSize.toString())
        .then((page) => res.json(page))
        .catch(next);
    });
}
