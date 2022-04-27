import {
  Box,
  Button,
  FormControl,
  FormControlLabel,
  Modal,
  Radio,
  RadioGroup,
} from "@mui/material";
import React from "react";
import { Elections } from "./contract";

function VotingModal({
  open,
  closeVotingModal,
  candidates,
  contract,
  electionId,
}: {
  open: boolean;
  closeVotingModal: () => void;
  candidates: string[];
  contract: Elections;
  electionId: number;
}) {
  const [choice, setChoice] = React.useState<string>("");

  const handleRadioChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setChoice((event.target as HTMLInputElement).value);
  };

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    contract.vote(BigInt(electionId), +choice).then(() => close());
    setChoice("");
  };

  const close = () => {
    closeVotingModal();
  };

  return (
    <Modal
      open={open}
      onClose={close}
      aria-labelledby="modal-modal-title"
      aria-describedby="modal-modal-description"
    >
      <Box
        sx={{
          position: "absolute" as "absolute",
          top: "50%",
          left: "50%",
          transform: "translate(-50%, -50%)",
          width: 400,
          bgcolor: "background.paper",
          border: "2px solid #000",
          boxShadow: 24,
          p: 4,
        }}
      >
        <form onSubmit={handleSubmit}>
          <FormControl>
            <RadioGroup value={choice} onChange={handleRadioChange}>
              {candidates.map((name, i) => (
                <FormControlLabel
                  key={i}
                  value={i}
                  control={<Radio />}
                  label={name}
                />
              ))}
            </RadioGroup>
            <Button type="submit" variant="outlined" disabled={!choice}>
              Vote
            </Button>
          </FormControl>
        </form>
      </Box>
    </Modal>
  );
}

export default VotingModal;
