import { useGba } from "@/hooks/useGba";
import CPURegisterView from "./CPURegisterView";
import IORegisterView from "./IORegisterView";

export type RegisterSubMenu = "cpu" | "i/o";

function RegisterView(props: { sub: RegisterSubMenu }) {
  const { cpu, memory } = useGba();

  if (props.sub === "cpu")
    return <CPURegisterView value={cpu.getRegisters()} style="full" />;

  if (props.sub === "i/o")
    return <IORegisterView value={memory.getIoRegisters()} style="full" />;
}

export default RegisterView;
