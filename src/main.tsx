import React from "react";
import ReactDOM from "react-dom/client";
import i18next from "i18next";
import { initReactI18next } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";

import { App } from "./App";
import translations from "./translations";
import { Api } from "./services/api";
import { Storage } from "./services/storage";
import { Services } from "./services";

i18next.use(initReactI18next).init({
  lng: navigator.language,
  fallbackLng: Object.keys(translations)[0],
  resources: translations,
});

const api = new Api(invoke);
const storage = new Storage();
const services = new Services(api, storage);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App services={services} />
  </React.StrictMode>,
);
