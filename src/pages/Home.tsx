import { Button, Stack } from "@mui/material";
import { BasicLayout } from "../components/BasicLayout";
import { useTranslation } from "react-i18next";

export function Home() {
  const { t } = useTranslation();

  const handleLaunchClick = () => {
    window.location.href = "/noname";
  };

  const launchButton = (
    <Button variant="contained" onClick={handleLaunchClick}>
      {t("home.launch")}
    </Button>
  );

  return (
    <BasicLayout>
      <Stack alignItems="center" spacing={2}>
        {launchButton}
      </Stack>
    </BasicLayout>
  );
}
