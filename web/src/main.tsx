import "@mantine/core/styles.css";

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { GbaContextProvider } from "./contexts/gba-context.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <GbaContextProvider>
      <App />
    </GbaContextProvider>
  </StrictMode>,
);
