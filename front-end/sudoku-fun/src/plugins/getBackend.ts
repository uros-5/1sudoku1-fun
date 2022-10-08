export function backend(): string {
  let prod = import.meta.env.PROD;
  return prod ? "https://1sudoku1.fun/w/" : "http://localhost:9000/";
}

export function wsUrl(): string {
  let prod = import.meta.env.PROD;
  let ws = prod ? "wss" : "ws";
  let http = prod ? "https" : "http";
  let b = (backend() as string).toString();
  let s = b.split(`${http}://`)[1];
  return `${ws}://${s}ws/`;
}
