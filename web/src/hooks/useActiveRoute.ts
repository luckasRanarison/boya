import { useMemo } from "react";
import { useLocation } from "react-router";
import ROUTES from "@/routes";

export function useActiveRoute() {
  const { pathname } = useLocation();

  const result = useMemo(() => {
    const segments = pathname.split("/").filter(Boolean);
    const lastSegment = segments[segments.length - 1];

    if (!lastSegment) {
      return { route: ROUTES[0], parent: undefined };
    }

    const activeRoute =
      ROUTES.find((r) => r.path === lastSegment) ||
      ROUTES.flatMap((r) => ("sub" in r ? r.sub : [])).find(
        (s) => s?.path === lastSegment,
      );

    return { route: activeRoute, parent: segments[segments.length - 2] };
  }, [pathname]);

  return result;
}
