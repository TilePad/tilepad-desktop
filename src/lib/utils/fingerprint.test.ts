import { it, expect, describe } from "vitest";

import { fingerprint } from "./fingerprint";

describe("fingerprint", () => {
  it("should return a SHA-256 fingerprint in colon-separated hex format", async () => {
    const key = new TextEncoder().encode("hello world");
    const result = await fingerprint(key);

    // Should be a string of hex bytes separated by ':'
    expect(result).toMatch(/^([0-9a-f]{2}:){31}[0-9a-f]{2}$/);
  });

  it("should produce consistent results for the same input", async () => {
    const key = new TextEncoder().encode("test key");
    const f1 = await fingerprint(key);
    const f2 = await fingerprint(key);
    expect(f1).toBe(f2);
  });

  it("should produce different results for different inputs", async () => {
    const key1 = new TextEncoder().encode("key1");
    const key2 = new TextEncoder().encode("key2");

    const f1 = await fingerprint(key1);
    const f2 = await fingerprint(key2);

    expect(f1).not.toBe(f2);
  });

  it("should handle empty input", async () => {
    const empty = new Uint8Array();
    const result = await fingerprint(empty);

    expect(result).toMatch(/^([0-9a-f]{2}:){31}[0-9a-f]{2}$/);
  });
});
