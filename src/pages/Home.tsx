import {
  Button,
  Card,
  CardContent,
  CardHeader,
  Container,
  Stack,
} from "@mui/material";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { ROUTES } from "../utils/consts";

export function Home() {
  const { t } = useTranslation();

  return (
    <Container sx={{ height: "100vh" }}>
      <Stack sx={{ height: "100%", justifyContent: "center" }}>
        <Card>
          <CardHeader title={t("home.title")} />
          <CardContent>
            <Stack gap={1}>
              <Link to={ROUTES.noname}>
                <Button variant="contained" fullWidth>
                  {t("game.noname.name")}
                </Button>
              </Link>
            </Stack>
          </CardContent>
        </Card>
      </Stack>
    </Container>
  );
}
