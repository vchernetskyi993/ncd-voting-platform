import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import React from "react";
import { Elections } from "./contract";
import ElectionRow from "./ElectionRow";

function ElectionsTable({
  openVotingModal,
  contract,
}: {
  openVotingModal: (
    candidates: string[],
    electionId: number
  ) => void;
  contract: Elections;
}) {
  const [electionIds, setElectionIds] = React.useState<number[]>();
  if (electionIds === undefined) {
    contract.electionsCount()
      .then((count) => {
        setElectionIds([...Array(Number(count)).keys()]);
      });
  }

  return (
    <TableContainer>
      <Table>
        <colgroup>
          <col width="20%" />
          <col width="35%" />
          <col width="35%" />
          <col width="10%" />
        </colgroup>
        <TableHead>
          <TableRow>
            <TableCell>
              <Typography variant="h5">ID</Typography>
            </TableCell>
            <TableCell>
              <Typography variant="h5">Description</Typography>
            </TableCell>
            <TableCell>
              <Typography variant="h5">Results</Typography>
            </TableCell>
            <TableCell></TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {electionIds?.map((electionId) => (
            <ElectionRow
              key={electionId}
              electionId={electionId}
              contract={contract!}
              openVotingModal={openVotingModal}
            />
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}

export default ElectionsTable;
