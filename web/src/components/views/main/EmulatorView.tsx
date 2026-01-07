import { useDebuggerStore } from "@/stores/debuggerStore";
import { AppShell, Flex, Group, Stack, Text } from "@mantine/core";
import { useEffect, useRef } from "react";

function EmulatorView() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { running, fps, run, setCanvas } = useDebuggerStore();

  useEffect(() => {
    if (canvasRef.current) {
      setCanvas(canvasRef.current);
    }
  }, [setCanvas]);

  useEffect(() => {
    run();
  }, [run]);

  return (
    <Stack flex={1}>
      <Flex h="100%" p="xl" align="center" justify="center">
        <canvas
          ref={canvasRef}
          style={{ imageRendering: "pixelated", border: "1px solid red" }}
          width={240}
          height={160}
        />
      </Flex>
      <AppShell.Footer p="md">
        <Group justify="space-between">
          {running ? (
            <Text c="green">Running</Text>
          ) : (
            <Text c="yellow">Paused</Text>
          )}
          <Text c="gray">
            {fps}
            /60 FPS
          </Text>
        </Group>
      </AppShell.Footer>
    </Stack>
  );
}

export default EmulatorView;
