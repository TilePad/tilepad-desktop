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
}
