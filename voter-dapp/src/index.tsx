import CssBaseline from "@mui/material/CssBaseline";
import { ThemeProvider } from "@mui/material/styles";
import ReactDOM from "react-dom/client";
import App from "./App";
import theme from "./theme";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import { getWallet } from "./blockchain";
import { getElectionsContract } from "./elections/contract";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

getWallet().then(async (wallet) => {
  if (!wallet.isSignedIn()) {
    await wallet.requestSignIn(process.env.REACT_APP_ELECTIONS_CONTRACT_ID!);
  }
  root.render(
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <App
        contract={getElectionsContract(wallet.account())}
        accountId={wallet.getAccountId()}
      />
    </ThemeProvider>
  );
});
