import {
  Button,
  Paper,
  Stack,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableRow,
  CircularProgress,
} from "@mui/material";

import { BasicLayout } from "../components/BasicLayout";
import { useTranslation } from "react-i18next";
import { useEffect, useState } from "react";
import { Noname } from "../services/api/noname";
import { Services } from "../services";

interface Props {
  services: Services;
}

export function Home(props: Props) {
  const [noname, setNoname] = useState<Noname | null>(null);

  const { t } = useTranslation();

  const handleLaunchClick = () => {
    window.location.href = "/noname/index.html";
  };

  useEffect(() => {
    props.services.api.noname.index().then((resp) => {
      setNoname(resp);
      props.services.storage.noname.set(resp);
    });
  }, [props.services.api.noname, props.services.storage.noname]);

  const infoTable = noname ? (
    <TableContainer component={Paper}>
      <Table>
        <TableBody>
          <TableRow>
            <TableCell>{t("home.info.path")}</TableCell>
            <TableCell
              sx={{
                color: (th) => th.palette.grey[200],
                background: (th) => th.palette.grey[800],
              }}
            >
              {noname.path}
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </TableContainer>
  ) : (
    <CircularProgress />
  );

  const launchButton = (
    <Button variant="contained" disabled={!noname} onClick={handleLaunchClick}>
      {t("home.launch")}
    </Button>
  );

  return (
    <BasicLayout>
      <Stack alignItems="center" spacing={2}>
        {infoTable}
        {launchButton}
      </Stack>
    </BasicLayout>
  );
}
