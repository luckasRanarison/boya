import { Flex, Loader as MantineLoader } from "@mantine/core";

function Loader() {
  return (
    <Flex flex={1} align="center" justify="center">
      <MantineLoader />
    </Flex>
  );
}

export default Loader;
