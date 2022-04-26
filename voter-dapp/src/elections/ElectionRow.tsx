import { Button, TableCell, TableRow, Typography } from "@mui/material";
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
  if (!election) {
    contract.getElection(BigInt(electionId)).then((e) => setElection(e));
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
        {/* TODO: disable button if:
            * election hasn't started 
            * election has ended
            * user already voted
            Add corresponding messages.
         */}
        <Button
          onClick={() =>
            openVotingModal(
              election?.candidates.map(({ name }) => name) || [],
              electionId
            )
          }
          variant="outlined"
        >
          Vote
        </Button>
      </TableCell>
    </TableRow>
  );
}

export default ElectionRow;
