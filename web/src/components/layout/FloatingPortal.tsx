import { useDebuggerMenu } from "@/hooks/useDebuggerMenu";
import { useGba } from "@/hooks/useGba";
import { useViewActions, useViewStore } from "@/stores/viewStore";
import FloatingWindow from "../common/FloatingWindow";
import { Portal } from "@mantine/core";
import DebuggerControls from "../views/debugger/DebuggerControls";

function FloatingPortal() {
  const state = useGba();
  const menus = useDebuggerMenu(state);
  const panelPosition = useViewStore((state) => state.debugPanelPosition);
  const floatingWindows = useViewStore((state) => state.floatingWindows);
  const { toggleWindow } = useViewActions();

  return (
    <>
      {floatingWindows.includes("panel") && (
        <DebuggerControls position={panelPosition} />
      )}

      {menus
        .filter(({ key }) => floatingWindows.includes(key))
        .map(({ key, label, view }) => (
          <Portal key={key}>
            <FloatingWindow title={label} onClose={() => toggleWindow(key)}>
              {view}
            </FloatingWindow>
          </Portal>
        ))}
    </>
  );
}

export default FloatingPortal;
