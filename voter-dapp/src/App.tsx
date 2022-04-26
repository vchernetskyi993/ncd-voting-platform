import { Box } from "@mui/material";
import { Elections } from "./elections/contract";
import React from "react";
import ElectionsTable from "./elections/ElectionsTable";
import Title from "./elections/Title";
import VotingModal from "./elections/VotingModal";

function App({
  contract,
  accountId,
}: {
  contract: Elections;
  accountId: string;
}) {
  const [open, setOpen] = React.useState(false);
  const [candidates, setCandidates] = React.useState<string[]>([]);
  const [electionId, setElectionId] = React.useState<number>();
  const openVotingModal = (
    candidates: string[],
    electionId: number
  ) => {
    setOpen(true);
    setCandidates(candidates);
    setElectionId(electionId);
  };
  const closeVotingModal = () => {
    setOpen(false);
    setCandidates([]);
  };

  return (
    <Box>
      <Title account={accountId} />
      <ElectionsTable
        contract={contract}
        openVotingModal={openVotingModal}
      />
      <VotingModal
        open={open}
        closeVotingModal={closeVotingModal}
        candidates={candidates}
        contract={contract}
        electionId={electionId!}
      />
    </Box>
  );
}

export default App;
