import { useDebuggerStore } from "@/stores/debuggerStore";
import { usePersistantStore } from "@/stores/persistantStore";
import { Flex, Stack } from "@mantine/core";
import { useEffect, useRef } from "react";
import EmulatorFooter from "./EmulatorFooter";

function EmulatorView() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { paused, run, setCanvas, createKeyHandler } = useDebuggerStore();
  const { keymap } = usePersistantStore();

  const handleKey = createKeyHandler(keymap);

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
      <EmulatorFooter />
    </Stack>
  );
}

export default EmulatorView;
