import { reducer } from "@/hooks/use-toast";

type ToasterToast = {
  id: string;
  title?: string;
  description?: string;
  open?: boolean;
};

type State = {
  toasts: ToasterToast[];
};

describe("use-toast reducer", () => {
  const emptyState: State = { toasts: [] };

  describe("ADD_TOAST", () => {
    it("adds a toast to an empty state", () => {
      const toast: ToasterToast = { id: "1", title: "Hello" };
      const result = reducer(emptyState, {
        type: "ADD_TOAST",
        toast: toast as any,
      });
      expect(result.toasts).toHaveLength(1);
      expect(result.toasts[0].id).toBe("1");
      expect(result.toasts[0].title).toBe("Hello");
    });

    it("prepends new toast (newest first)", () => {
      const state: State = {
        toasts: [{ id: "1", title: "First" } as any],
      };
      const result = reducer(state, {
        type: "ADD_TOAST",
        toast: { id: "2", title: "Second" } as any,
      });
      expect(result.toasts[0].id).toBe("2");
    });

    it("enforces the toast limit (TOAST_LIMIT = 1)", () => {
      const state: State = {
        toasts: [{ id: "1", title: "First" } as any],
      };
      const result = reducer(state, {
        type: "ADD_TOAST",
        toast: { id: "2", title: "Second" } as any,
      });
      expect(result.toasts).toHaveLength(1);
      expect(result.toasts[0].id).toBe("2");
    });
  });

  describe("UPDATE_TOAST", () => {
    it("updates the matching toast by id", () => {
      const state: State = {
        toasts: [{ id: "1", title: "Old Title" } as any],
      };
      const result = reducer(state, {
        type: "UPDATE_TOAST",
        toast: { id: "1", title: "New Title" } as any,
      });
      expect(result.toasts[0].title).toBe("New Title");
    });

    it("does not modify non-matching toasts", () => {
      const state: State = {
        toasts: [{ id: "1", title: "Keep Me" } as any],
      };
      const result = reducer(state, {
        type: "UPDATE_TOAST",
        toast: { id: "99", title: "Should Not Apply" } as any,
      });
      expect(result.toasts[0].title).toBe("Keep Me");
    });

    it("merges partial updates into the existing toast", () => {
      const state: State = {
        toasts: [
          { id: "1", title: "Title", description: "Desc" } as any,
        ],
      };
      const result = reducer(state, {
        type: "UPDATE_TOAST",
        toast: { id: "1", description: "Updated Desc" } as any,
      });
      expect(result.toasts[0].title).toBe("Title");
      expect(result.toasts[0].description).toBe("Updated Desc");
    });
  });

  describe("DISMISS_TOAST", () => {
    it("sets open to false for a specific toast", () => {
      const state: State = {
        toasts: [{ id: "1", title: "Toast", open: true } as any],
      };
      const result = reducer(state, {
        type: "DISMISS_TOAST",
        toastId: "1",
      });
      expect(result.toasts[0].open).toBe(false);
    });

    it("dismisses all toasts when no toastId is provided", () => {
      const state: State = {
        toasts: [
          { id: "1", open: true } as any,
          { id: "2", open: true } as any,
        ],
      };
      const result = reducer(state, {
        type: "DISMISS_TOAST",
      });
      result.toasts.forEach((t: any) => {
        expect(t.open).toBe(false);
      });
    });

    it("leaves non-targeted toasts open", () => {
      const state: State = {
        toasts: [
          { id: "1", open: true } as any,
          { id: "2", open: true } as any,
        ],
      };
      const result = reducer(state, {
        type: "DISMISS_TOAST",
        toastId: "1",
      });
      expect(result.toasts[0].open).toBe(false);
      expect(result.toasts[1].open).toBe(true);
    });
  });

  describe("REMOVE_TOAST", () => {
    it("removes a specific toast by id", () => {
      const state: State = {
        toasts: [
          { id: "1", title: "Stay" } as any,
          { id: "2", title: "Remove" } as any,
        ],
      };
      const result = reducer(state, {
        type: "REMOVE_TOAST",
        toastId: "2",
      });
      expect(result.toasts).toHaveLength(1);
      expect(result.toasts[0].id).toBe("1");
    });

    it("removes all toasts when no toastId is provided", () => {
      const state: State = {
        toasts: [
          { id: "1" } as any,
          { id: "2" } as any,
        ],
      };
      const result = reducer(state, {
        type: "REMOVE_TOAST",
        toastId: undefined,
      });
      expect(result.toasts).toHaveLength(0);
    });

    it("returns the same state when toast id does not exist", () => {
      const state: State = {
        toasts: [{ id: "1" } as any],
      };
      const result = reducer(state, {
        type: "REMOVE_TOAST",
        toastId: "99",
      });
      expect(result.toasts).toHaveLength(1);
    });
  });
});
