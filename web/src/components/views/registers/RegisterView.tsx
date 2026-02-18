import { useGba } from "@/hooks/useGba";
import CPURegisterView from "./CPURegisterView";
import IORegisterView from "./IORegisterView";
import { useParams } from "react-router";

export type RegisterSubMenu = "cpu" | "i/o";

function RegisterView() {
  const { cpu, memory } = useGba();
  const { type } = useParams<{ type: string }>();

  if (type === "cpu")
    return <CPURegisterView value={cpu.getRegisters()} style="full" />;

  if (type === "io")
    return <IORegisterView value={memory.getIoRegisters()} style="full" />;
}

export default RegisterView;
