import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";
import ROUTES from "./routes";
import MainLayout from "./components/layout/MainLayout";

const router = createBrowserRouter(
  [
    {
      path: "/",
      element: <MainLayout />,
      children: ROUTES.map((route) => ({
        path: route.path,
        Component: route.component,
        children: route.sub?.map((sub) => ({
          path: sub.path,
          Component: "component" in sub ? sub.component : route.component,
        })),
      })),
    },
  ],
  {
    basename: import.meta.env.BASE_URL,
  },
);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
