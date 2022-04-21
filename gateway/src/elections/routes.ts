import express, { Router } from "express";

export function electionsRouter(): Router {
  return express
    .Router()
    .post("/", (req, res) => {
      res.send("Creating election...");
    })
    .get("/:electionId", (req, res) => {
      res.send(`Getting election ${req.params.electionId} ...`);
    })
    .get("/", (req, res) => {
      res.send("Getting paginated elections...");
    });
}
