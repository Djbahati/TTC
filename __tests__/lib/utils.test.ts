import { cn } from "@/lib/utils";

describe("cn (className utility)", () => {
  it("merges class names", () => {
    expect(cn("foo", "bar")).toBe("foo bar");
  });

  it("handles conditional classes via clsx syntax", () => {
    expect(cn("base", false && "hidden", "visible")).toBe("base visible");
  });

  it("returns empty string when called with no arguments", () => {
    expect(cn()).toBe("");
  });

  it("deduplicates conflicting Tailwind classes (twMerge)", () => {
    expect(cn("px-2 py-1", "px-4")).toBe("py-1 px-4");
  });

  it("handles arrays of class names", () => {
    expect(cn(["foo", "bar"])).toBe("foo bar");
  });

  it("handles undefined and null values", () => {
    expect(cn("base", undefined, null, "end")).toBe("base end");
  });

  it("handles objects with boolean values", () => {
    expect(cn({ hidden: true, visible: false })).toBe("hidden");
  });

  it("merges conflicting Tailwind text colors", () => {
    expect(cn("text-red-500", "text-blue-500")).toBe("text-blue-500");
  });

  it("merges conflicting Tailwind background colors", () => {
    expect(cn("bg-white", "bg-neutral-900")).toBe("bg-neutral-900");
  });

  it("preserves non-conflicting Tailwind classes", () => {
    expect(cn("p-4 text-white", "bg-black")).toBe("p-4 text-white bg-black");
  });
});
