import { Route, Routes } from "react-router";
import { BrowserRouter } from "react-router";
import { useRuntimeStore } from "@/stores/runtimeStore";
import MainLayout from "./components/layout/MainLayout";
import EmulatorView from "./components/views/main/EmulatorView";
import UploadView from "./components/views/main/UploadView";
import ObjectView from "./components/views/objects/ObjectView";
import BackgroundView from "./components/views/backgrounds/BackgroundView";
import AboutView from "./components/views/about/AboutView";
import DebuggerView from "./components/views/debugger/DebuggerView";
import SettingsView from "./components/views/settings/SettingsView";
import RegisterView from "./components/views/registers/RegisterView";
import MemoryView from "./components/views/memory/MemoryView";

function App() {
  const romLoaded = useRuntimeStore((state) => state.romLoaded);

  return (
    <BrowserRouter basename={import.meta.env.BASE_URL}>
      <Routes>
        <Route element={<MainLayout />}>
          <Route
            index
            element={romLoaded ? <EmulatorView /> : <UploadView />}
          />

          <Route path="objects" element={<ObjectView />} />
          <Route path="backgrounds" element={<BackgroundView />} />
          <Route path="about" element={<AboutView />} />
          <Route path="debugger" element={<DebuggerView />} />
          <Route path="settings" element={<SettingsView />} />
          <Route path="memory/:region" element={<MemoryView />} />
          <Route path="registers/:type" element={<RegisterView />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
