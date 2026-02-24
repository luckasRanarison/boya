import RomCard from "@/components/common/RomCard";
import { useRuntimeStore } from "@/stores/runtimeStore";
import { Stack, Text } from "@mantine/core";

function AboutView() {
  const rom = useRuntimeStore((state) => state.rom);

  return (
    <Stack py="lg" px="md">
      {rom ? (
        <RomCard data={rom} />
      ) : (
        <Text mt="md" ta="center">
          🚧 This is a work in progress... 🚧
        </Text>
      )}
    </Stack>
  );
}

export default AboutView;
