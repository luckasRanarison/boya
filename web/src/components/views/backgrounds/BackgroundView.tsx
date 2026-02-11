import Tile from "@/components/common/Tile";
import { useGba } from "@/hooks/useGba";
import { Box, Card, SimpleGrid, Stack, Text, Tooltip } from "@mantine/core";
import { useMemo } from "react";
import { FlagList } from "../registers/FlagList";

function BackgroundView() {
  const backgrounds = [0, 1, 2, 3];
  const { memory, renderBg } = useGba();

  const bgcnts = useMemo(() => {
    const registers = memory.getIoRegisters();
    const index = registers.findIndex((r) => r.name === "BGCNT0");
    return registers.slice(index, index + 4);
  }, [memory]);

  return (
    <Stack flex={1} px="md" py="xl" justify="center" align="center">
      <SimpleGrid spacing="xl" cols={{ base: 1, md: 2 }}>
        {backgrounds.map((bg) => (
          <Stack key={bg} align="center">
            <Tooltip
              bg="none"
              label={
                <Card p="md">
                  <FlagList value={bgcnts[bg].value} flags={bgcnts[bg].flags} />
                </Card>
              }
            >
              <Box>
                <Tile
                  render={() => renderBg(bg)}
                  width={240 * 1.5}
                  height={160 * 1.5}
                  innerWidth={240}
                  innerHeight={160}
                />
              </Box>
            </Tooltip>
            <Text size="lg">{bg}</Text>
          </Stack>
        ))}
      </SimpleGrid>
    </Stack>
  );
}

export default BackgroundView;
