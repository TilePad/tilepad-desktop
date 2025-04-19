type UnlockFunction = () => void;
type Resolver = (unlock: UnlockFunction) => void;

export class Mutex {
  private locked = false;
  private waiting: Resolver[] = [];

  async lock(): Promise<UnlockFunction> {
    const unlock = () => {
      const next = this.waiting.shift();
      if (next) {
        next(unlock);
      } else {
        this.locked = false;
      }
    };

    if (this.locked) {
      return new Promise<UnlockFunction>((resolve) => {
        this.waiting.push(resolve);
      }).then(() => unlock);
    } else {
      this.locked = true;
      return Promise.resolve(unlock);
    }
  }

  async runExclusive<T>(callback: () => Promise<T> | T): Promise<T> {
    const unlock = await this.lock();
    try {
      return await Promise.resolve(callback());
    } finally {
      unlock();
    }
  }
}
