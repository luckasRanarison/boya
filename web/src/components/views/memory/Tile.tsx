import { instance } from "@/lib/gba";
import { ColorMode } from "boya_wasm";
import { useEffect, useRef } from "react";

function Tile(props: {
  rawData: Uint8Array;
  paletteId: number;
  mode: ColorMode;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!canvasRef.current) return;

    canvasRef.current.width = 8;
    canvasRef.current.height = 8;

    const ctx = canvasRef.current.getContext("2d")!;
    const imageData = ctx.createImageData(8, 8);

    instance.writeTileBuffer(
      imageData.data as unknown as Uint8Array,
      props.rawData,
      props.mode,
      props.paletteId,
    );

    ctx.putImageData(imageData, 0, 0);
  }, [props.rawData, props.paletteId, props.mode]);

  return (
    <canvas
      ref={canvasRef}
      style={{
        height: 40,
        width: 40,
        imageRendering: "pixelated",
        border: "1px solid gray",
      }}
    />
  );
}

export default Tile;
