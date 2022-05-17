import { getWallet } from "./blockchain/utils";
import { Elm } from "./Main.elm";

document.addEventListener("DOMContentLoaded", () => {
  getWallet().then(async (wallet) => {
    if (!wallet.isSignedIn()) {
      await wallet.requestSignIn(process.env.ELECTIONS_CONTRACT_ID!);
    }
    const app = Elm.Main.init({
      node: document.getElementById("app"),
      flags: {
        accountId: wallet.getAccountId(),
      },
    });
    app.ports.interopFromElm.subscribe((fromElm) => {
      switch (fromElm.tag) {
        case "alert": {
          alert(fromElm.data.message);
          break;
        }
      }
    });
  });
});
