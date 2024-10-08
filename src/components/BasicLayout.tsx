import { Box, Stack } from "@mui/material";
import { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

export function BasicLayout(props: Props) {
  return (
    <Box
      sx={{
        height: "100vh",
        padding: 1,
        background: (th) => th.palette.background.paper,
      }}
    >
      <Stack>{props.children}</Stack>
    </Box>
  );
}
