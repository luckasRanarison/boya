import { useReducer } from "react";

type PageState = {
  pageId: number;
};

type PageAction =
  | {
      type: "jump";
      address: number;
    }
  | { type: "select"; pageId: number };

type PageCtx = {
  pageSize: number;
  offset: number;
};

function pageReducer(state: PageState, action: PageAction, ctx: PageCtx) {
  switch (action.type) {
    case "select":
      return { pageId: action.pageId };

    case "jump": {
      const pageId =
        Math.floor((action.address - ctx.offset) / ctx.pageSize) + 1;

      return { pageId };
    }

    default:
      return state;
  }
}

export const useMemoryPage = (ctx: PageCtx) =>
  useReducer(
    (state: PageState, action: PageAction) => pageReducer(state, action, ctx),
    { pageId: 1 },
  );
