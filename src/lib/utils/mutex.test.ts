import { it, expect, describe } from "vitest";

import { Mutex } from "./mutex";

describe("Mutex", () => {
  it("should allow one lock at a time", async () => {
    const mutex = new Mutex();
    let running = false;

    const first = mutex.runExclusive(async () => {
      running = true;
      await new Promise((r) => setTimeout(r, 50));
      running = false;
      return "first";
    });

    const second = mutex.runExclusive(async () => {
      expect(running).toBe(false); // should not overlap
      return "second";
    });

    await expect(first).resolves.toBe("first");
    await expect(second).resolves.toBe("second");
  });

  it("should queue multiple locks and resolve them in order", async () => {
    const mutex = new Mutex();
    const results: string[] = [];

    const tasks = [1, 2, 3].map((i) =>
      mutex.runExclusive(async () => {
        results.push(`start-${i}`);
        await new Promise((r) => setTimeout(r, 10));
        results.push(`end-${i}`);
        return i;
      }),
    );

    const values = await Promise.all(tasks);
    expect(values).toEqual([1, 2, 3]);
    expect(results).toEqual([
      "start-1",
      "end-1",
      "start-2",
      "end-2",
      "start-3",
      "end-3",
    ]);
  });

  it("should release the lock after unlock() is called", async () => {
    const mutex = new Mutex();

    const unlock = await mutex.lock();
    let acquired = false;

    const waitLock = mutex.lock().then(() => {
      acquired = true;
      return () => {};
    });

    expect(acquired).toBe(false);

    unlock(); // release lock
    await waitLock;

    expect(acquired).toBe(true);
  });

  it("should propagate errors from runExclusive callback", async () => {
    const mutex = new Mutex();

    const err = new Error("boom");
    await expect(
      mutex.runExclusive(async () => {
        throw err;
      }),
    ).rejects.toThrow("boom");

    // Next runExclusive should still work
    await expect(mutex.runExclusive(() => "ok")).resolves.toBe("ok");
  });

  it("should handle synchronous callbacks in runExclusive", async () => {
    const mutex = new Mutex();
    const result = await mutex.runExclusive(() => 42);
    expect(result).toBe(42);
  });

  it("should allow re-locking after unlock in multiple cycles", async () => {
    const mutex = new Mutex();
    const results: number[] = [];

    for (let i = 0; i < 3; i++) {
      await mutex.runExclusive(async () => {
        results.push(i);
      });
    }

    expect(results).toEqual([0, 1, 2]);
  });
});
