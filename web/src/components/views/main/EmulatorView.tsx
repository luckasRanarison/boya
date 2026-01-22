import { useDebuggerStore } from "@/stores/debuggerStore";
import { Flex, Stack } from "@mantine/core";
import { useEffect, useRef } from "react";
import EmulatorFooter from "./EmulatorFooter";
import { useGamepadHandler } from "@/hooks/useGamepadHandler";

function EmulatorView() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { paused, run, setCanvas } = useDebuggerStore();
  const handleKey = useGamepadHandler();

  useEffect(() => {
    if (canvasRef.current) {
      setCanvas(canvasRef.current);
    }
  }, [setCanvas]);

  useEffect(() => {
    document.addEventListener("keydown", handleKey);
    document.addEventListener("keyup", handleKey);

    return () => {
      document.removeEventListener("keydown", handleKey);
      document.removeEventListener("keyup", handleKey);
    };
  }, [run, handleKey]);

  useEffect(() => {
    if (!paused) {
      run();
    }
  }, [paused, run]);

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
