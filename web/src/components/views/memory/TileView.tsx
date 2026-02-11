import { Box, Card, Flex, Select, SimpleGrid, Stack } from "@mantine/core";
import { useState } from "react";
import { GBA } from "@/lib/gba";
import { ColorMode } from "boya_wasm";

import ColorView from "./ColorView";
import Tile from "@/components/common/Tile";

const tileConfig = {
  "4bpp": { tileSize: 32, paletteSize: 16 },
  "8bpp": { tileSize: 64, paletteSize: 256 },
};

type TileMode = "4bpp" | "8bpp";

function TileView(props: { pageStart: number; pageData: Uint8Array }) {
  const [currentMode, setCurrentMode] = useState<TileMode>("4bpp");
  const [currentPaletteId, setCurrentPaletteId] = useState(0);
  const { tileSize, paletteSize } = tileConfig[currentMode];
  const colorPalette = GBA.colorPalette();

  const palettes = colorPalette.reduce<number[][]>((prev, _curr, i) => {
    if (i % paletteSize === 0) {
      prev.push(Array.from(colorPalette.slice(i, i + paletteSize)));
    }

    return prev;
  }, []);

  const tiles = props.pageData.reduce<Uint8Array[]>((prev, _curr, i) => {
    if (i % tileSize === 0) {
      prev.push(props.pageData.slice(i, i + tileSize));
    }

    return prev;
  }, []);

  return (
    <Flex
      py="xl"
      px={{ base: "sm", md: "xl" }}
      w="100%"
      h="100%"
      align="center"
      direction={{ base: "column", md: "row" }}
      gap="xl"
      style={{ overflow: "scroll" }}
    >
      <SimpleGrid cols={8} spacing={0} mx="auto">
        {tiles.map((_, id) => (
          <Tile
            key={id}
            width={40}
            height={40}
            innerWidth={8}
            innerHeight={8}
            render={() =>
              GBA.renderTileBuffer(
                id,
                props.pageStart,
                currentMode === "4bpp"
                  ? ColorMode.Palette16
                  : ColorMode.Palette256,
                currentPaletteId,
              )
            }
          />
        ))}
      </SimpleGrid>

      <Card mb="auto" withBorder>
        <Stack>
          <Select
            label="Color mode"
            value={currentMode}
            data={Object.keys(tileConfig)}
            onChange={(m) => m && setCurrentMode(m as TileMode)}
          />
          <Stack gap="0" mah="200" style={{ overflow: "scroll" }}>
            {palettes.map((p, id) => (
              <Box
                key={id}
                onClick={() => setCurrentPaletteId(id)}
                style={{ cursor: "pointer" }}
              >
                <ColorView value={p} columns={16} />
              </Box>
            ))}
          </Stack>
        </Stack>
      </Card>
    </Flex>
  );
}

export default TileView;
