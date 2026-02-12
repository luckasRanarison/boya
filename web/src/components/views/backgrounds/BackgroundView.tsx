import Tile from "@/components/common/Tile";
import { useGba } from "@/hooks/useGba";
import {
  Box,
  Card,
  Stack,
  Text,
  Tooltip,
  ActionIcon,
  Group,
} from "@mantine/core";
import { IconLayoutGrid, IconLayersIntersect } from "@tabler/icons-react";
import { useMemo, useState } from "react";
import { FlagList } from "../registers/FlagList";

function BackgroundView() {
  const backgrounds = [0, 1, 2, 3];
  const { memory, renderBg } = useGba();
  const [flat, setFlat] = useState(false);
  const [mode, setMode] = useState<"stack" | "grid">("stack");

  const bgcnts = useMemo(() => {
    const registers = memory.getIoRegisters();
    const index = registers.findIndex((r) => r.name === "BGCNT0");
    return registers.slice(index, index + 4);
  }, [memory]);

  const sorted = useMemo(() => {
    return backgrounds
      .map((bg) => ({
        bg,
        prio: bgcnts[bg].value & 0b11,
      }))
      .sort((a, b) => a.prio - b.prio);
  }, [bgcnts]);

  const isGrid = mode === "grid" && flat;

  return (
    <Stack flex={1} px="md" py="xl" pos="relative">
      <Group
        justify="flex-end"
        pos="absolute"
        top={10}
        right={20}
        style={{ zIndex: 100 }}
      >
        <ActionIcon
          variant="light"
          size="lg"
          onClick={(e) => {
            e.stopPropagation();
            setMode(mode === "stack" ? "grid" : "stack");
            setFlat(mode === "stack");
          }}
          title={
            mode === "stack" ? "Switch to Grid View" : "Switch to Stack View"
          }
        >
          {mode === "stack" ? (
            <IconLayoutGrid size={20} />
          ) : (
            <IconLayersIntersect size={20} />
          )}
        </ActionIcon>
      </Group>

      <Box
        onClick={() => mode === "stack" && setFlat((f) => !f)}
        style={{
          display: isGrid ? "grid" : "block",
          gridTemplateColumns: isGrid ? "1fr 1fr" : "none",
          gap: isGrid ? "20px" : "0",
          position: "relative",
          width: isGrid ? "fit-content" : 240 * 1.5,
          height: isGrid ? "fit-content" : 160 * 1.5,
          perspective: 1000,
          margin: "auto",
          transition: "all 400ms ease",
        }}
      >
        {sorted.map(({ bg, prio }, index) => {
          const depth = sorted.length - index;

          return (
            <Box
              key={bg}
              style={{
                position: isGrid ? "relative" : "absolute",
                top: flat ? (isGrid ? 0 : undefined) : "30%",
                transformStyle: "preserve-3d",
                transition: "all 400ms ease",
                zIndex: depth,
                transform: flat
                  ? "none"
                  : `rotateX(55deg) translateZ(${depth * 40}px) translateY(${depth * -15}px)`,
                opacity: flat ? 1 : 0.9,
              }}
            >
              <Tooltip
                bg="none"
                label={
                  <Card p="md" withBorder>
                    <FlagList
                      value={bgcnts[bg].value}
                      flags={bgcnts[bg].flags}
                    />
                  </Card>
                }
              >
                <Box>
                  <Tile
                    render={() => renderBg(bg)}
                    width={isGrid ? 240 : 240 * 1.5}
                    height={isGrid ? 160 : 160 * 1.5}
                    innerWidth={240}
                    innerHeight={160}
                  />
                </Box>
              </Tooltip>

              {(isGrid || !flat) && (
                <Text
                  size="xs"
                  fw={700}
                  style={{
                    position: "absolute",
                    top: isGrid ? -20 : -30,
                    left: 0,
                  }}
                >
                  BG {bg} (prio {prio})
                </Text>
              )}
            </Box>
          );
        })}
      </Box>
    </Stack>
  );
}

export default BackgroundView;
