import {
  Button,
  Group,
  Input,
  Modal,
  SimpleGrid,
  Stack,
  Text,
  Tooltip,
} from "@mantine/core";
import { GBA } from "@/lib/gba";
import { formatHex, parseHex } from "@/utils/format";
import ColorView from "./ColorView";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useMemo, useState } from "react";
import { IconArrowRight } from "@tabler/icons-react";
import notifications from "@/lib/notifications";
import Loader from "@/components/common/Loader";

type ByteLine = {
  address: number;
  columns: number[];
  right:
    | { type: "color"; value: number[] }
    | { type: "ascii"; value: number[] };
};

type Edit = {
  address: number;
  prev: number;
};

function HexView(props: {
  pageData: Uint8Array;
  baseAddress: number;
  pageStart: number;
  columns: number;
  rightSection?: "color" | "ascii";
}) {
  const [edit, setEdit] = useState<Edit | null>(null);
  const [opened, { open, close }] = useDisclosure();
  const [loading, setLoading] = useState(true);

  const lines = useMemo(() => {
    const lines: ByteLine[] = [];
    const colors = props.rightSection === "color" ? GBA.colorPalette() : null;

    for (let i = 0; i < props.pageData.length; i += props.columns) {
      const row = props.pageData.slice(i, i + props.columns);
      const bytes = Array.from(row);

      lines.push({
        address: props.baseAddress + props.pageStart + i,
        columns: bytes,
        right: colors
          ? {
              type: "color",
              value: Array.from(colors.slice(i / 2, i / 2 + 8)),
            }
          : { type: "ascii", value: bytes },
      });
    }

    return lines;
  }, [props]);

  const handleEdit = (address: number, prev: number) => {
    setEdit({ address, prev });
    open();
  };

  const handleConfirm = (value: string) => {
    const parsed = parseHex(value);

    if (edit && !Number.isNaN(parsed)) {
      GBA.writeByte(edit.address, parsed);
      notifications.info(`${formatHex(edit.address)} has been written!`);
      setEdit(null);
    } else {
      notifications.error("Invalid address");
    }
  };

  useEffect(() => {
    setTimeout(() => setLoading(false), 10);
  }, []);

  if (loading) {
    return <Loader />;
  }

  return (
    <Stack p="xl" w="100%" ff={"monospace"} align="center">
      {edit && (
        <EditModal
          address={edit.address}
          prev={edit.prev}
          opened={opened}
          onClose={close}
          onConfirm={handleConfirm}
        />
      )}

      {lines.map((line) => (
        <Group key={line.address} w="100%" justify="space-between">
          <Text c="indigo" fw={600}>
            {formatHex(line.address)}:
          </Text>
          <SimpleGrid
            spacing="md"
            cols={{ base: 8, sm: 16 }}
            w={{ base: "100%", sm: "auto" }}
          >
            {line.columns.map((byte, idx) => {
              const address = line.address + idx;

              return (
                <Tooltip key={address} label={formatHex(address)}>
                  <Text
                    id={`${formatHex(address)}`}
                    c="gray"
                    ta="center"
                    onClick={() => handleEdit(address, byte)}
                  >
                    {byte.toString(16).padStart(2, "0")}
                  </Text>
                </Tooltip>
              );
            })}
          </SimpleGrid>

          {line.right.type === "color" && (
            <ColorView value={line.right.value} columns={8} />
          )}

          {line.right.type === "ascii" && (
            <Text w={`${props.columns}ch`} c="indigo.4">
              {line.right.value
                .map((b) =>
                  b >= 32 && b <= 126 ? String.fromCharCode(b) : ".",
                )
                .join("")}
            </Text>
          )}
        </Group>
      ))}
    </Stack>
  );
}

function EditModal(props: {
  address: number;
  prev: number;
  opened: boolean;
  onClose: () => void;
  onConfirm: (value: string) => void;
}) {
  const [value, setValue] = useState<string>("");
  const error = Number.isNaN(parseHex(value));

  return (
    <Modal
      title="Edit Memory"
      size="xs"
      opened={props.opened}
      onClose={props.onClose}
      centered
    >
      <form
        onSubmit={(e) => {
          e.preventDefault();
          props.onConfirm(value);
          setValue("");
        }}
      >
        <Stack gap="lg">
          <Group justify="space-between" gap="xs" ff="monospace" fz="lg">
            <Text fw={700} c="indigo">
              {formatHex(props.address)}:
            </Text>

            <Text c="dimmed">{formatHex(props.prev, { width: 2 })}</Text>

            <IconArrowRight size={16} style={{ opacity: 0.5 }} />

            <Input
              variant="filled"
              placeholder="XX"
              w={60}
              autoFocus
              value={value}
              onChange={(e) => setValue(e.currentTarget.value.toUpperCase())}
              error={value && error}
              styles={{
                input: { textAlign: "center", fontFamily: "monospace" },
              }}
            />
          </Group>

          <Group grow>
            <Button type="submit" disabled={!value || error}>
              Confirm
            </Button>
          </Group>
        </Stack>
      </form>
    </Modal>
  );
}

export default HexView;
