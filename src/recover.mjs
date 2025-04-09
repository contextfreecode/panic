export function recover(onError, body) {
  try {
    return body();
  } catch (error) {
    return onError(error);
  }
}
