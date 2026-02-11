import { getFlagValue } from "@/utils/bitflag";
import { Stack, Group, Text } from "@mantine/core";
import type { Flag } from "boya_wasm";

export function FlagList({
  flags,
  value,
  showBits,
}: {
  value: number;
  flags: Flag[];
  showBits?: boolean;
}) {
  return (
    <Stack>
      {flags
        .filter((f) => f.name !== "unused")
        .map((flag) => {
          const flagValue = getFlagValue(value, flag);

          return (
            <Group key={flag.name + flag.start}>
              {showBits && (
                <Text c="indigo" size="sm" w="7ch">
                  [
                  {flag.length === 1
                    ? flag.start
                    : `${flag.start}-${flag.start + flag.length - 1}`}
                  ]
                </Text>
              )}
              <Group flex={1}>
                <Text size="sm">{flag.name}:</Text>
                <Text size="sm" c="gray">
                  {flag.mappings
                    ? `${flagValue} (${flag.mappings[flagValue]})`
                    : flagValue}
                </Text>
              </Group>
            </Group>
          );
        })}
    </Stack>
  );
}
