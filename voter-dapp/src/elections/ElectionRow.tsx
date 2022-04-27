import {
  Button,
  TableCell,
  TableRow,
  Tooltip,
  Typography,
} from "@mui/material";
import React from "react";
import { Election, Elections } from "./contract";

function ElectionRow({
  electionId,
  contract,
  openVotingModal,
}: {
  electionId: number;
  contract: Elections;
  openVotingModal: (candidates: string[], electionId: number) => void;
}) {
  const [election, setElection] = React.useState<Election>();
  const [canVote, setCanVote] = React.useState(false);
  const [helpMessage, setHelpMessage] = React.useState("Loading...");
  if (!election) {
    contract.getElection(BigInt(electionId)).then((e) => setElection(e));
  }
  if (election && helpMessage === "Loading...") {
    contract.haveVoted(BigInt(electionId)).then((voted) => {
      if (voted) {
        setHelpMessage("You've already voted.");
      } else if (toMillis(election.start) > Date.now()) {
        setHelpMessage("Election hasn't started yet.");
      } else if (toMillis(election.end) < Date.now()) {
        setHelpMessage("Election has already finished.")
      } else {
        setHelpMessage("");
        setCanVote(true);
      }
    });
  }
  return (
    <TableRow>
      <TableCell>
        <Typography variant="h6">{electionId}</Typography>
      </TableCell>
      <TableCell>
        <Typography variant="h6">{election?.title}</Typography>
        <Typography variant="subtitle2">{election?.description}</Typography>
      </TableCell>
      <TableCell>
        {election?.candidates.map(({ name, votes }, i) => (
          <Typography key={name}>{`${name} (${votes})`} </Typography>
        )) || []}
      </TableCell>
      <TableCell>
        <Tooltip title={canVote ? "" : helpMessage}>
          <span>
            <Button
              onClick={() =>
                openVotingModal(
                  election?.candidates.map(({ name }) => name) || [],
                  electionId
                )
              }
              variant="outlined"
              disabled={!canVote}
            >
              Vote
            </Button>
          </span>
        </Tooltip>
      </TableCell>
    </TableRow>
  );
}

function toMillis(nanoseconds: string): number {
  return Number(BigInt(nanoseconds) / 1_000_000n);
}

export default ElectionRow;
