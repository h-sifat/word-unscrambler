export const enum Status {
  IDLE = "idle",
  ERROR = "error",
  RUNNING = "running",
  COMPLETED = "completed",
}

export function shuffleStr(str: string) {
  const chars = str.split("");

  for (let i = 0; i < chars.length; i++) {
    const j = Math.floor(Math.random() * chars.length);
    swap(chars, i, j);
  }

  return chars.join("");
}

function swap(arr: any[], i: number, j: number) {
  let temp = arr[i];

  arr[i] = arr[j];
  arr[j] = temp;
}
