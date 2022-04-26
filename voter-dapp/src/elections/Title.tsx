import { AppBar, Toolbar, Typography } from "@mui/material";

function Title({ account }: { account: string | undefined }) {
  return (
    <AppBar position="sticky">
      <Toolbar>
        <Typography variant="h4" sx={{ flexGrow: 1 }}>
          Elections
        </Typography>
        <Typography>{account}</Typography>
      </Toolbar>
    </AppBar>
  );
}

export default Title;
