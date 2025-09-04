import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

// Experimental feature warning
console.warn(
  "%c⚠️ EXPERIMENTAL FEATURE ⚠️",
  "color: orange; font-size: 16px; font-weight: bold;"
);
console.warn(
  "This WSL Terminal application is experimental and may contain bugs.\n" +
    "Features may change or be removed in future versions."
);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
