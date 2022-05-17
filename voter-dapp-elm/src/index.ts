import { Elm } from "./Main.elm";

document.addEventListener("DOMContentLoaded", () => {
  const app = Elm.Main.init({
    node: document.getElementById("app"),
    flags: null,
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
