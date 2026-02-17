import { useEffect, useRef } from "react";

function Tile(props: {
  render: () => Uint8Array;
  innerWidth: number;
  innerHeight: number;
  width: number;
  height: number;
  checkerboard?: boolean;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!canvasRef.current) return;

    canvasRef.current.width = props.innerWidth;
    canvasRef.current.height = props.innerHeight;

    const ctx = canvasRef.current.getContext("2d")!;
    const imageData = ctx.createImageData(props.innerWidth, props.innerHeight);
    const buffer = props.render();

    for (let i = 0; i < buffer.length; i++) {
      imageData.data[i] = buffer[i];
    }

    ctx.putImageData(imageData, 0, 0);
  }, [props]);

  return (
    <canvas
      ref={canvasRef}
      style={{
        height: props.height,
        width: props.width,
        imageRendering: "pixelated",
        border: "1px solid var(--mantine-color-default-border)",
        background: props.checkerboard
          ? "repeating-conic-gradient(#f0f0f0 0% 25%, #fff 0% 50%) 50% / 16px 16px"
          : undefined,
      }}
    />
  );
}

export default Tile;
