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
  TextField,
  FormControlLabel,
  Checkbox,
  Divider,
} from "@mui/material";

import { BasicLayout } from "../components/BasicLayout";
import { useTranslation } from "react-i18next";
import { useEffect, useState } from "react";
import {
  NonameStatusResponse,
  NonameUpdateStatus,
  NonameUpdateStatusResponse,
} from "../services/api/noname";
import { Services } from "../services";

interface Props {
  services: Services;
}

export function Home(props: Props) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [noname, setNoname] = useState<NonameStatusResponse | null>(null);
  const [repo, setRepo] = useState("https://github.com/libccy/noname.git");
  const [branch, setBranch] = useState("master");
  const [checkTimer, setCheckTimer] = useState<number | null>(null);
  const [updateStatus, setUpdateStatus] =
    useState<NonameUpdateStatusResponse | null>(null);
  const [expose, setExpose] = useState(false);

  const { t } = useTranslation();

  const handleUpdateClick = () => {
    setLoading(true);
    props.services.api.noname
      .update({ repo, branch })
      .then(() => {
        setCheckTimer(
          setInterval(() => {
            props.services.api.noname.update_status().then((resp) => {
              if (typeof resp === "object") {
                setError(resp.Err);
                if (checkTimer !== null) clearInterval(checkTimer);
                setCheckTimer(null);
                setUpdateStatus(null);
                setLoading(false);
              } else if (resp === NonameUpdateStatus.Ok) {
                window.location.reload();
              } else {
                setUpdateStatus(resp);
              }
            });
          }, 1000),
        );
      })
      .catch((err) => {
        setError(JSON.stringify(err));
        setLoading(false);
      });
  };

  const handleLaunchClick = () => {
    setLoading(true);
    props.services.api.noname
      .launch({ expose })
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

  const updateProgress = (
    <Collapse in={!!updateStatus} sx={{ width: "100%" }}>
      <Alert severity="info">
        {t("home.update.updating")}{" "}
        {typeof updateStatus === "string" ? updateStatus : "N/A"}
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
          <TableRow>
            <TableCell>{t("home.info.updated_at")}</TableCell>
            <TableCell
              sx={{
                color: (th) => th.palette.grey[200],
                background: (th) => th.palette.grey[800],
              }}
            >
              {(noname.updated_at && new Date(noname.updated_at).toString()) ||
                "N/A"}
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </TableContainer>
  ) : (
    <CircularProgress />
  );

  const updateForm = (
    <Stack spacing={2} sx={{ width: "100%" }}>
      <TextField
        label={t("home.update.repo")}
        value={repo}
        onChange={(ev) => setRepo(ev.target.value)}
      />
      <TextField
        label={t("home.update.branch")}
        value={branch}
        onChange={(ev) => setBranch(ev.target.value)}
      />
      <Button
        variant="contained"
        color="warning"
        disabled={loading}
        onClick={handleUpdateClick}
      >
        {t("home.update.submit")}
      </Button>
    </Stack>
  );

  const launchForm = (
    <Stack spacing={2} sx={{ width: "100%" }}>
      <FormControlLabel
        control={
          <Checkbox
            checked={expose}
            onChange={(ev) => setExpose(ev.target.checked)}
          />
        }
        label={t("home.launch.expose")}
      />
      <Button
        variant="contained"
        disabled={loading || !noname || !noname.updated_at}
        onClick={handleLaunchClick}
        sx={{ width: "100%" }}
      >
        {t("home.launch.submit")}
      </Button>
    </Stack>
  );

  return (
    <BasicLayout>
      <Stack alignItems="center" spacing={2}>
        {errorAlert}
        {updateProgress}
        {infoTable}
        <Divider sx={{ width: "100%", color: (th) => th.palette.grey[800] }} />
        {updateForm}
        <Divider sx={{ width: "100%", color: (th) => th.palette.grey[800] }} />
        {launchForm}
      </Stack>
    </BasicLayout>
  );
}
