import Tile from "@/components/common/Tile";
import { useGba } from "@/hooks/useGba";
import {
  Box,
  Stack,
  Text,
  ActionIcon,
  SimpleGrid,
  Modal,
  Flex,
  Card,
} from "@mantine/core";
import {
  IconLayoutGrid,
  IconLayersIntersect,
  IconEye,
  IconEyeOff,
} from "@tabler/icons-react";
import { useMemo, useState, useRef } from "react";
import { FlagList } from "../registers/FlagList";

function BackgroundView() {
  const { memory, renderBg } = useGba();
  const [mode, setMode] = useState<"stack" | "grid">("grid");
  const [focused, setFocused] = useState<number | null>(null);
  const [flat, setFlat] = useState(true);
  const [wasMoving, setWasMoving] = useState(false);
  const [hiddenBgs, setHiddenBgs] = useState<Record<number, boolean>>({});

  const [rotation, setRotation] = useState({ x: 55, y: 0 });
  const [isDragging, setIsDragging] = useState(false);
  const lastMousePos = useRef({ x: 0, y: 0 });

  const bgcnt = useMemo(() => {
    const registers = memory.getIoRegisters();
    const index = registers.findIndex((r) => r.name === "BGCNT0");
    return registers.slice(index, index + 4);
  }, [memory]);

  const sorted = useMemo(() => {
    return bgcnt
      .map((bg, index) => ({
        bg: index,
        prio: bg.value & 0b11,
      }))
      .sort((a, b) => a.prio - b.prio);
  }, [bgcnt]);

  const toggleBgVisibility = (bgIndex: number) => {
    setHiddenBgs((prev) => ({ ...prev, [bgIndex]: !prev[bgIndex] }));
  };

  const isGrid = mode === "grid" && flat;

  const handleMouseDown = (e: React.PointerEvent) => {
    if (mode === "grid") return;
    setIsDragging(true);
    lastMousePos.current = { x: e.clientX, y: e.clientY };
  };

  const handleMouseMove = (e: React.PointerEvent) => {
    if (!isDragging) return;

    const deltaX = e.clientX - lastMousePos.current.x;
    const deltaY = e.clientY - lastMousePos.current.y;

    setRotation((prev) => ({
      x: prev.x - deltaY * 0.5,
      y: prev.y + deltaX * 0.5,
    }));

    setWasMoving(true);
    lastMousePos.current = { x: e.clientX, y: e.clientY };
  };

  const stopDragging = () => {
    setIsDragging(false);
    setTimeout(() => {
      setWasMoving(false);
    }, 100);
  };

  return (
    <Stack
      flex={1}
      px="md"
      py="xl"
      style={{ position: "relative", overflow: "hidden" }}
    >
      {mode === "stack" && (
        <Card
          withBorder
          p="xs"
          style={{ zIndex: 50, position: "absolute", top: 20, right: 20 }}
        >
          <Stack gap="xs">
            <Text size="xs" fw="bold" c="dimmed">
              LAYERS
            </Text>
            {sorted.map(({ bg, prio }) => (
              <Flex key={bg} align="center" gap="sm">
                <ActionIcon
                  size="sm"
                  variant="subtle"
                  color={hiddenBgs[bg] ? "gray" : "blue"}
                  onClick={(e) => {
                    e.stopPropagation();
                    toggleBgVisibility(bg);
                  }}
                >
                  {hiddenBgs[bg] ? (
                    <IconEyeOff size={14} />
                  ) : (
                    <IconEye size={14} />
                  )}
                </ActionIcon>
                <Text size="xs" fw={600} style={{ whiteSpace: "nowrap" }}>
                  BG {bg}{" "}
                  <Text span c="dimmed" fw={400}>
                    (Prio {prio})
                  </Text>
                </Text>
              </Flex>
            ))}
          </Stack>
        </Card>
      )}

      {focused !== null && (
        <Modal
          title={`BG ${focused}`}
          onClose={() => setFocused(null)}
          opened
          centered
        >
          <Stack gap="xl">
            <Flex justify="center">
              <Tile
                render={() => renderBg(focused)}
                width={240}
                height={160}
                innerWidth={240}
                innerHeight={160}
              />
            </Flex>
            <FlagList
              value={bgcnt[focused].value}
              flags={bgcnt[focused].flags}
            />
          </Stack>
        </Modal>
      )}

      <ActionIcon
        title="Toggle view mode"
        size="lg"
        variant="filled"
        style={{
          position: "fixed",
          bottom: 25,
          right: 25,
          zIndex: 50,
        }}
        onClick={() => {
          setMode(mode === "stack" ? "grid" : "stack");
          setFlat(mode === "stack");
          setRotation({ x: 55, y: 0 });
        }}
      >
        {mode === "stack" ? (
          <IconLayoutGrid size={20} />
        ) : (
          <IconLayersIntersect size={20} />
        )}
      </ActionIcon>

      <Box
        onPointerDown={handleMouseDown}
        onPointerMove={handleMouseMove}
        onPointerUp={stopDragging}
        onPointerLeave={stopDragging}
        style={{
          flex: 1,
          touchAction: mode === "stack" ? "none" : "auto",
          cursor: isDragging
            ? "grabbing"
            : mode === "stack" && !flat
              ? "grab"
              : "default",
          userSelect: "none",
          perspective: 1200,
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
        onClick={() => {
          if (mode === "stack" && !wasMoving) setFlat((prev) => !prev);
        }}
      >
        <SimpleGrid
          cols={{ base: 1, lg: 2 }}
          spacing="xl"
          verticalSpacing="sm"
          style={{
            position: "relative",
            width: isGrid ? "fit-content" : 240 * 1.2,
            height: isGrid ? "fit-content" : 160 * 1.2,
            transformStyle: "preserve-3d",
          }}
        >
          {sorted.map(({ bg, prio }, index) => {
            const depth = sorted.length - index;
            const isHidden = !!hiddenBgs[bg];

            return (
              <Flex
                direction={isGrid ? "column" : "column-reverse"}
                gap="xs"
                align="center"
                key={bg}
                style={{
                  position: isGrid ? "relative" : "absolute",
                  top: flat ? (isGrid ? undefined : 0) : "30%",
                  transformStyle: "preserve-3d",
                  zIndex: depth,
                  transition:
                    isDragging || mode === "grid"
                      ? undefined
                      : "450ms ease-in-out",
                  transform: flat
                    ? undefined
                    : `
                        rotateX(${rotation.x}deg)
                        rotateY(${rotation.y}deg)
                        translateZ(${depth * 40}px)
                        translateY(${depth * -15}px)
                      `,
                  opacity: isGrid ? 1 : isHidden ? 0.1 : 0.9,
                  pointerEvents: isGrid ? "auto" : "none",
                }}
                onClick={() => mode === "grid" && setFocused(bg)}
              >
                <Box
                  style={{
                    visibility: isHidden ? "hidden" : "visible",
                  }}
                >
                  <Tile
                    render={() => renderBg(bg)}
                    width={isGrid ? 240 : 240 * 1.2}
                    height={isGrid ? 160 : 160 * 1.2}
                    innerWidth={240}
                    innerHeight={160}
                  />
                </Box>

                {isGrid && (
                  <Text size="xs" fw={700}>
                    BG {bg} (prio {prio})
                  </Text>
                )}
              </Flex>
            );
          })}
        </SimpleGrid>
      </Box>
    </Stack>
  );
}

export default BackgroundView;
