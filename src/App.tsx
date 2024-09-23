import { createBrowserRouter, RouterProvider } from "react-router-dom";

import "./App.css";
import { Home } from "./pages/Home";
import { ROUTES } from "./utils/consts";
import { Services } from "./services";
import { ThemeProvider } from "@emotion/react";
import { theme } from "./theme";
import { CssBaseline } from "@mui/material";

interface Props {
  services: Services;
}

export function App(props: Props) {
  const router = createBrowserRouter([
    {
      path: ROUTES.home,
      element: <Home services={props.services} />,
    },
  ]);

  return (
    <ThemeProvider theme={theme}>
      <>
        <CssBaseline />
        <RouterProvider router={router} />
      </>
    </ThemeProvider>
  );
}
