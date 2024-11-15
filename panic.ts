// https://youtu.be/AdmGHwvgaVs

async function wait(duration: number) {
  return new Promise((resolve) => {
    setTimeout(resolve, duration);
  });
}

async function getUser(id: number) {
  await wait(0); // or longer
  if (id === 2) {
    throw new NotFoundError("404 - User does not exist");
  }
  return { id, name: "Kyle" };
}

function catchError<T>(promise: Promise<T>): Promise<[undefined, T] | [Error]> {
  return promise
    .then((data) => {
      return [undefined, data] as [undefined, T];
    })
    .catch((error) => {
      return [error];
    });
}

function catchErrorTyped<T, E extends new (message?: string) => Error>(
  promise: Promise<T>,
  errorsToCatch?: E[]
): Promise<[undefined, T] | [InstanceType<E>]> {
  return promise
    .then((data) => {
      return [undefined, data] as [undefined, T];
    })
    .catch((error) => {
      if (errorsToCatch == undefined) {
        return [error];
      }
      if (errorsToCatch.some((e) => error instanceof e)) {
        return [error];
      }
      throw error;
    });
}

class NotFoundError extends Error {
  name = "NotFoundError";
  extraProp = "ERROR: test";
}

async function demoCatchAll(id: number) {
  try {
    const user = await getUser(id);
    console.log(user);
  } catch (error) {
    console.log("There was an error:", error.message);
  }
}

async function demoHelper(id: number) {
  const [error, user] = await catchError(getUser(id));
  if (error) {
    console.log("There was an error:", error.message);
  } else {
    console.log(user);
  }
}

async function demoHelperTyped(id: number) {
  const [error, user] = await catchErrorTyped(getUser(id), [NotFoundError]);
  if (error) {
    console.log("There was an error:", error.message);
  } else {
    console.log(user);
  }
}

async function main() {
  const id = 1;
  demoCatchAll(id);
  demoHelper(id);
  demoHelperTyped(id);
}

main();

export {};
