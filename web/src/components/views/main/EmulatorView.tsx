import { useRuntimeActions, useRuntimeStore } from "@/stores/runtimeStore";
import { Stack, ActionIcon, Box } from "@mantine/core";
import { useEffect, useRef, useState } from "react";
import EmulatorFooter from "./EmulatorFooter";
import { useViewActions } from "@/stores/viewStore";
import { useBreakpoints } from "@/stores/debuggerStore";
import { GBA } from "@/lib/gba";
import {
  IconPlus,
  IconMinus,
  IconMinimize,
  IconMaximize,
} from "@tabler/icons-react";
import { useFullscreen } from "@mantine/hooks";
import FpsCounter from "./FpsCounter";
import { usePersistantStore } from "@/stores/persistantStore";
import OnScreenControls from "./OnScreenControls";

function EmulatorView() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [scale, setScale] = useState(1.5);
  const { toggle, fullscreen, ref: containerRef } = useFullscreen();
  const smoothFilter = usePersistantStore((state) => state.smoothFilter);

  const paused = useRuntimeStore((state) => state.paused);
  const breakpoints = useBreakpoints();
  const { run } = useRuntimeActions();
  const { setCanvas, renderFrame } = useViewActions();

  useEffect(() => {
    if (canvasRef.current) {
      setCanvas(canvasRef.current);
      renderFrame(GBA);
    }
  }, [setCanvas, renderFrame]);

  useEffect(() => {
    if (!paused) {
      run({ onFrame: renderFrame, breakpoints });
    }
  }, [paused, breakpoints, run, renderFrame]);

  return (
    <Stack
      ref={containerRef}
      align="center"
      justify="center"
      style={{ flex: 1, position: "relative", overflow: "hidden" }}
    >
      <ActionIcon.Group
        orientation="vertical"
        style={{
          position: "absolute",
          top: fullscreen ? 60 : 20,
          right: 20,
          zIndex: 10,
        }}
      >
        <ActionIcon
          title="Zoom in"
          variant="default"
          onClick={() => setScale((s) => Math.min(s + 0.5, 5))}
          aria-label="Zoom In"
        >
          <IconPlus size={16} />
        </ActionIcon>
        <ActionIcon
          title="Zoom out"
          variant="default"
          onClick={() => setScale((s) => Math.max(s - 0.5, 1))}
          aria-label="Zoom Out"
        >
          <IconMinus size={16} />
        </ActionIcon>
        <ActionIcon
          title="Toggle fullscreen"
          variant="default"
          onClick={toggle}
          style={{ borderTop: 0 }}
        >
          {fullscreen ? <IconMinimize size={18} /> : <IconMaximize size={18} />}
        </ActionIcon>
      </ActionIcon.Group>

      <canvas
        ref={canvasRef}
        style={{
          imageRendering: smoothFilter ? "smooth" : "pixelated",
          border: "1px solid var(--mantine-color-red-5)",
          transform: `scale(${scale})`,
        }}
        width={240}
        height={160}
      />

      <OnScreenControls fullscreen={fullscreen} />

      {fullscreen ? (
        <Box style={{ position: "absolute", top: 20, right: 20 }}>
          <FpsCounter color="red" />
        </Box>
      ) : (
        <EmulatorFooter canvas={() => canvasRef.current} />
      )}
    </Stack>
  );
}

export default EmulatorView;
