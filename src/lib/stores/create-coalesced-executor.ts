export function createCoalescedExecutor<T, R>(fn: (value: T) => Promise<R>) {
  let inFlight = false;
  let pending:
    | {
        value: T;
        deferreds: Array<{ resolve: (value: R) => void; reject: (error: unknown) => void }>;
      }
    | null = null;

  async function run(value: T): Promise<R> {
    if (inFlight) {
      return new Promise<R>((resolve, reject) => {
        if (pending) {
          pending.value = value;
          pending.deferreds.push({ resolve, reject });
          return;
        }

        pending = {
          value,
          deferreds: [{ resolve, reject }],
        };
      });
    }

    inFlight = true;

    try {
      let currentValue = value;
      let queuedDeferreds: Array<{ resolve: (value: R) => void; reject: (error: unknown) => void }> =
        [];

      while (true) {
        try {
          const result = await fn(currentValue);

          for (const deferred of queuedDeferreds) {
            deferred.resolve(result);
          }

          if (!pending) {
            return result;
          }

          currentValue = pending.value;
          queuedDeferreds = pending.deferreds;
          pending = null;
        } catch (error) {
          for (const deferred of queuedDeferreds) {
            deferred.reject(error);
          }

          if (pending) {
            for (const deferred of pending.deferreds) {
              deferred.reject(error);
            }

            pending = null;
          }

          throw error;
        }
      }
    } finally {
      inFlight = false;
    }
  }

  return run;
}
