export function floatingPositions(isMobile: boolean) {
  const panelWidth = 370; // debug panel's approximate width
  const halfWidth = panelWidth / 2;

  return {
    "up-left": {
      top: "10%",
      left: "2%",
    },
    up: {
      top: "10%",
      left: isMobile ? `calc(50% - ${halfWidth}px)` : "50%",
    },
    "up-right": {
      top: "10%",
      right: "2%",
    },
    left: {
      top: "50%",
      left: "2%",
    },
    center: {
      top: "50%",
      left: isMobile ? `calc(50% - ${halfWidth}px)` : "50%",
    },
    right: {
      top: "50%",
      right: "2%",
    },
    "down-left": {
      bottom: "10%",
      left: "2%",
    },
    down: {
      bottom: "10%",
      left: isMobile ? `calc(50% - ${halfWidth}px)` : "50%",
    },
    "down-right": {
      bottom: "10%",
      right: "2%",
    },
  };
}

export type Position = keyof ReturnType<typeof floatingPositions>;
