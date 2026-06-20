import { renderHook, act } from "@testing-library/react";
import { useIsMobile } from "@/hooks/use-mobile";

describe("useIsMobile", () => {
  let listeners: Array<() => void> = [];
  let matchMediaMock: jest.Mock;

  beforeEach(() => {
    listeners = [];
    matchMediaMock = jest.fn().mockImplementation((query: string) => ({
      matches: false,
      media: query,
      addEventListener: (_event: string, handler: () => void) => {
        listeners.push(handler);
      },
      removeEventListener: (_event: string, handler: () => void) => {
        listeners = listeners.filter((l) => l !== handler);
      },
    }));
    Object.defineProperty(window, "matchMedia", {
      writable: true,
      value: matchMediaMock,
    });
  });

  it("returns false when window width is above mobile breakpoint", () => {
    Object.defineProperty(window, "innerWidth", {
      writable: true,
      value: 1024,
    });
    const { result } = renderHook(() => useIsMobile());
    expect(result.current).toBe(false);
  });

  it("returns true when window width is below mobile breakpoint", () => {
    Object.defineProperty(window, "innerWidth", {
      writable: true,
      value: 500,
    });
    const { result } = renderHook(() => useIsMobile());
    expect(result.current).toBe(true);
  });

  it("returns true when window width is exactly at breakpoint - 1", () => {
    Object.defineProperty(window, "innerWidth", {
      writable: true,
      value: 767,
    });
    const { result } = renderHook(() => useIsMobile());
    expect(result.current).toBe(true);
  });

  it("returns false when window width is exactly at breakpoint", () => {
    Object.defineProperty(window, "innerWidth", {
      writable: true,
      value: 768,
    });
    const { result } = renderHook(() => useIsMobile());
    expect(result.current).toBe(false);
  });

  it("responds to media query change events", () => {
    Object.defineProperty(window, "innerWidth", {
      writable: true,
      value: 1024,
    });
    const { result } = renderHook(() => useIsMobile());
    expect(result.current).toBe(false);

    act(() => {
      Object.defineProperty(window, "innerWidth", {
        writable: true,
        value: 500,
      });
      listeners.forEach((listener) => listener());
    });
    expect(result.current).toBe(true);
  });

  it("cleans up event listener on unmount", () => {
    Object.defineProperty(window, "innerWidth", {
      writable: true,
      value: 1024,
    });
    const { unmount } = renderHook(() => useIsMobile());
    const listenerCountBefore = listeners.length;
    unmount();
    expect(listeners.length).toBeLessThan(listenerCountBefore);
  });

  it("calls matchMedia with the correct query", () => {
    Object.defineProperty(window, "innerWidth", {
      writable: true,
      value: 1024,
    });
    renderHook(() => useIsMobile());
    expect(matchMediaMock).toHaveBeenCalledWith("(max-width: 767px)");
  });
});
