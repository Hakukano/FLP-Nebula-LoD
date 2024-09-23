import React from "react";
import ReactDOM from "react-dom/client";
import { CssBaseline, ThemeProvider } from "@mui/material";
import i18next from "i18next";
import { initReactI18next } from "react-i18next";

import App from "./App";
import { theme } from "./theme";
import translations from "./translations";

i18next.use(initReactI18next).init({
  lng: navigator.language,
  fallbackLng: Object.keys(translations)[0],
  resources: translations,
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <>
        <CssBaseline />
        <App />
      </>
    </ThemeProvider>
  </React.StrictMode>,
);
