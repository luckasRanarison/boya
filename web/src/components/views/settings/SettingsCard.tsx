import { Box, Divider, Paper, Stack, Title } from "@mantine/core";

function SettingsCard({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <Paper withBorder p="md" radius="md">
      <Stack gap="md">
        <Box>
          <Title order={5} c="indigo" fw="bold">
            {title}
          </Title>
          <Divider mt="xs" />
        </Box>
        {children}
      </Stack>
    </Paper>
  );
}

export default SettingsCard;
