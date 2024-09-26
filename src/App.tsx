import "./App.css";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { CssBaseline } from "@mui/material";
import { ThemeProvider } from "@emotion/react";

import { Git } from "./pages/Git";
import { ROUTES } from "./utils/consts";
import { Services } from "./services";
import { Home } from "./pages/Home";
import { theme } from "./theme";

interface Props {
  services: Services;
}

export function App(props: Props) {
  const router = createBrowserRouter([
    {
      path: ROUTES.home,
      element: <Home />,
    },
    {
      path: ROUTES.noname,
      element: <Git services={props.services} name="noname" />,
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
