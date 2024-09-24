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
  Divider,
  Grid2,
  FormControl,
  FormLabel,
} from "@mui/material";

import { BasicLayout } from "../components/BasicLayout";
import { useTranslation } from "react-i18next";
import { useCallback, useEffect, useState } from "react";
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
  const [updateStatus, setUpdateStatus] =
    useState<NonameUpdateStatusResponse | null>(null);
  const [bindIp0, setBindIp0] = useState(127);
  const [bindIp1, setBindIp1] = useState(0);
  const [bindIp2, setBindIp2] = useState(0);
  const [bindIp3, setBindIp3] = useState(1);
  const [bindPort, setBindPort] = useState(19719);

  const { t } = useTranslation();

  let checkTimer: number | null = null;

  const fetchNoname = useCallback(() => {
    setLoading(true);
    props.services.api.noname
      .status()
      .then((resp) => {
        setNoname(resp);
      })
      .finally(() => setLoading(false));
  }, [props.services.api.noname]);

  const handleUpdateClick = () => {
    setLoading(true);
    props.services.api.noname
      .update({ repo, branch })
      .then(() => {
        checkTimer = setInterval(() => {
          props.services.api.noname.update_status().then((resp) => {
            if (typeof resp === "object") {
              setError(resp.Err);
            }
            if (typeof resp === "object" || resp === NonameUpdateStatus.Ok) {
              if (checkTimer !== null) clearInterval(checkTimer);
              checkTimer = null;
              setUpdateStatus(null);
              fetchNoname();
            } else {
              setUpdateStatus(resp);
            }
          });
        }, 1000);
      })
      .catch((err) => {
        setError(JSON.stringify(err));
        setLoading(false);
      });
  };

  const handleLaunchClick = () => {
    setLoading(true);
    props.services.api.noname
      .launch({
        bindAddress: `${bindIp0}.${bindIp1}.${bindIp2}.${bindIp3}:${bindPort}`,
      })
      .then((resp) => {
        window.location.href = resp;
      })
      .catch((err) => setError(JSON.stringify(err)))
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    fetchNoname();
  }, [fetchNoname]);

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
            <TableCell
              sx={{
                color: (th) => th.palette.grey[200],
                background: (th) => th.palette.grey[800],
              }}
            >
              {t("home.info.path")}
            </TableCell>
            <TableCell>{noname.path}</TableCell>
          </TableRow>
          <TableRow>
            <TableCell
              sx={{
                color: (th) => th.palette.grey[200],
                background: (th) => th.palette.grey[800],
              }}
            >
              {t("home.info.updated_at")}
            </TableCell>
            <TableCell>
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
      <Grid2 container spacing={1}>
        <Grid2 size={8}>
          <FormControl fullWidth>
            <FormLabel>{t("home.update.repo")}</FormLabel>
            <TextField
              value={repo}
              onChange={(ev) => setRepo(ev.target.value)}
            />
          </FormControl>
        </Grid2>
        <Grid2 size={4}>
          <FormControl fullWidth>
            <FormLabel>{t("home.update.branch")}</FormLabel>
            <TextField
              value={branch}
              onChange={(ev) => setBranch(ev.target.value)}
            />
          </FormControl>
        </Grid2>
      </Grid2>
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
      <Grid2 container spacing={1}>
        <Grid2 size={8}>
          <FormControl fullWidth>
            <FormLabel>{t("home.launch.bind_ip")}</FormLabel>
            <Stack direction="row">
              <TextField
                type="number"
                value={bindIp0}
                onChange={(ev) => setBindIp0(parseInt(ev.target.value))}
                sx={{ flexGrow: 1 }}
              />
              <TextField
                type="number"
                value={bindIp1}
                onChange={(ev) => setBindIp1(parseInt(ev.target.value))}
                sx={{ flexGrow: 1 }}
              />
              <TextField
                type="number"
                value={bindIp2}
                onChange={(ev) => setBindIp2(parseInt(ev.target.value))}
                sx={{ flexGrow: 1 }}
              />
              <TextField
                type="number"
                value={bindIp3}
                onChange={(ev) => setBindIp3(parseInt(ev.target.value))}
                sx={{ flexGrow: 1 }}
              />
            </Stack>
          </FormControl>
        </Grid2>
        <Grid2 size={4}>
          <FormControl fullWidth>
            <FormLabel>{t("home.launch.bind_port")}</FormLabel>
            <TextField
              type="number"
              value={bindPort}
              onChange={(ev) => setBindPort(parseInt(ev.target.value))}
            />
          </FormControl>
        </Grid2>
      </Grid2>
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
