import { useRuntimeStore } from "@/stores/runtimeStore";
import { Text, type MantineColor } from "@mantine/core";

function FpsCounter(props: { color?: MantineColor }) {
  const fps = useRuntimeStore((state) => state.fps);

  return <Text c={props.color ?? "gray"}>{fps} FPS</Text>;
}

export default FpsCounter;
