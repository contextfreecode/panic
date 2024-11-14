// https://youtu.be/AdmGHwvgaVs

async function wait(duration: number) {
  return new Promise((resolve) => {
    setTimeout(resolve, duration);
  });
}

async function getUser(id: number) {
  await wait(100);
  if (id === 2) {
    throw new Error("404 - User does not exist");
  }
  return { id, name: "Kyle" };
}

try {
  const user = await getUser(1);
  console.log(user);
} catch (error) {
  console.log("There was an error");
}

export {};
