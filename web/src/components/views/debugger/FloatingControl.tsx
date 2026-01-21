import { useState } from "react";
import {
  IconArrowDown,
  IconArrowDownLeft,
  IconArrowDownRight,
  IconArrowLeft,
  IconArrowRight,
  IconArrowUp,
  IconArrowUpLeft,
  IconArrowUpRight,
  IconCircle,
} from "@tabler/icons-react";
import { FloatingIndicator, UnstyledButton } from "@mantine/core";
import classes from "./FloatingControl.module.css";
import type { Position } from "@/stores/debuggerStore";

function FloatingControl(props: {
  defaultValue: Position;
  onChange: (position: Position) => void;
}) {
  const [rootRef, setRootRef] = useState<HTMLDivElement | null>(null);
  const [controlsRefs, setControlsRefs] = useState<
    Record<string, HTMLButtonElement | null>
  >({});
  const [active, setActive] = useState(props.defaultValue);

  const handleChange = (position: Position) => {
    setActive(position);
    props.onChange(position);
  };

  const setControlRef = (name: Position) => (node: HTMLButtonElement) => {
    controlsRefs[name] = node;
    setControlsRefs(controlsRefs);
  };

  return (
    <div className={classes.root} dir="ltr" ref={setRootRef}>
      <FloatingIndicator
        target={controlsRefs[active]}
        parent={rootRef}
        className={classes.indicator}
      />

      <div className={classes.controlsGroup}>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("up-left")}
          ref={setControlRef("up-left")}
          mod={{ active: active === "up-left" }}
        >
          <IconArrowUpLeft size={18} stroke={1.5} />
        </UnstyledButton>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("up")}
          ref={setControlRef("up")}
          mod={{ active: active === "up" }}
        >
          <IconArrowUp size={18} stroke={1.5} />
        </UnstyledButton>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("up-right")}
          ref={setControlRef("up-right")}
          mod={{ active: active === "up-right" }}
        >
          <IconArrowUpRight size={18} stroke={1.5} />
        </UnstyledButton>
      </div>
      <div className={classes.controlsGroup}>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("left")}
          ref={setControlRef("left")}
          mod={{ active: active === "left" }}
        >
          <IconArrowLeft size={18} stroke={1.5} />
        </UnstyledButton>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("center")}
          ref={setControlRef("center")}
          mod={{ active: active === "center" }}
        >
          <IconCircle size={18} stroke={1.5} />
        </UnstyledButton>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("right")}
          ref={setControlRef("right")}
          mod={{ active: active === "right" }}
        >
          <IconArrowRight size={18} stroke={1.5} />
        </UnstyledButton>
      </div>
      <div className={classes.controlsGroup}>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("down-left")}
          ref={setControlRef("down-left")}
          mod={{ active: active === "down-left" }}
        >
          <IconArrowDownLeft size={18} stroke={1.5} />
        </UnstyledButton>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("down")}
          ref={setControlRef("down")}
          mod={{ active: active === "down" }}
        >
          <IconArrowDown size={18} stroke={1.5} />
        </UnstyledButton>
        <UnstyledButton
          className={classes.control}
          onClick={() => handleChange("down-right")}
          ref={setControlRef("down-right")}
          mod={{ active: active === "down-right" }}
        >
          <IconArrowDownRight size={18} stroke={1.5} />
        </UnstyledButton>
      </div>
    </div>
  );
}

export default FloatingControl;
