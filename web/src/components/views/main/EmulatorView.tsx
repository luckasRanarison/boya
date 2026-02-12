import { useRuntimeActions, useRuntimeStore } from "@/stores/runtimeStore";
import { Flex, Stack } from "@mantine/core";
import { useEffect, useRef } from "react";
import EmulatorFooter from "./EmulatorFooter";
import { useGamepadHandler } from "@/hooks/useGamepadHandler";
import { useViewActions } from "@/stores/viewStore";
import { useBreakpoints } from "@/stores/debuggerStore";
import { GBA } from "@/lib/gba";

function EmulatorView() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const paused = useRuntimeStore((state) => state.paused);
  const breakpoints = useBreakpoints();
  const { run } = useRuntimeActions();
  const { setCanvas, renderFrame } = useViewActions();
  const handleKey = useGamepadHandler();

  useEffect(() => {
    if (canvasRef.current) {
      setCanvas(canvasRef.current);
      renderFrame(GBA);
    }
  }, [setCanvas, renderFrame]);

  useEffect(() => {
    document.addEventListener("keydown", handleKey);
    document.addEventListener("keyup", handleKey);

    return () => {
      document.removeEventListener("keydown", handleKey);
      document.removeEventListener("keyup", handleKey);
    };
  }, [handleKey]);

  useEffect(() => {
    if (!paused) {
      run({ onFrame: renderFrame, breakpoints });
    }
  }, [paused, breakpoints, run, renderFrame]);

  return (
    <Stack flex={1}>
      <Flex h="100%" p="xl" align="center" justify="center">
        <canvas
          ref={canvasRef}
          style={{
            imageRendering: "pixelated",
            border: "1px solid red",
          }}
          width={240}
          height={160}
        />
      </Flex>
      <EmulatorFooter canvas={() => canvasRef.current} />
    </Stack>
  );
}

export default EmulatorView;
