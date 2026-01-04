import UploadArea from "../UploadArea";
import { useView } from "../../stores/viewStore";
import BiosView from "../views/BiosView";

function Main() {
  const { view } = useView();

  if (view === "bios") return <BiosView />;

  return <UploadArea />;
}

export default Main;
