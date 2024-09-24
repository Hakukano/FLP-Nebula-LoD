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
  Alert,
  Collapse,
} from "@mui/material";

import { BasicLayout } from "../components/BasicLayout";
import { useTranslation } from "react-i18next";
import { useEffect, useState } from "react";
import { NonameStatusResponse } from "../services/api/noname";
import { Services } from "../services";

interface Props {
  services: Services;
}

export function Home(props: Props) {
  const [loading, setLoading] = useState(false);
  const [noname, setNoname] = useState<NonameStatusResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const { t } = useTranslation();

  const handleLaunchClick = () => {
    setLoading(true);
    props.services.api.noname
      .launch({ expose: true })
      .then((resp) => {
        window.location.href = resp;
      })
      .catch((err) => setError(JSON.stringify(err)))
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    setLoading(true);
    props.services.api.noname
      .status()
      .then((resp) => {
        setNoname(resp);
        props.services.storage.noname.set(resp);
      })
      .finally(() => setLoading(false));
  }, [props.services.api.noname, props.services.storage.noname]);

  const errorAlert = (
    <Collapse in={!!error} sx={{ width: "100%" }}>
      <Alert severity="error" onClose={() => setError(null)}>
        {error}
      </Alert>
    </Collapse>
  );

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
    <Button variant="contained" disabled={loading} onClick={handleLaunchClick}>
      {t("home.launch")}
    </Button>
  );

  return (
    <BasicLayout>
      <Stack alignItems="center" spacing={2}>
        {errorAlert}
        {infoTable}
        {launchButton}
      </Stack>
    </BasicLayout>
  );
}
