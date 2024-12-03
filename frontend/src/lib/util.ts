import { MAX_WORD_LEN } from "../config";
import { Status } from "./interface";

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

export function sortWordsBasedOnLength(words: string[]): [number, string[]][] {
  const sorted: Record<number, string[]> = {};

  for (const word of words) (sorted[word.length] ||= []).push(word);

  Object.values(sorted).forEach((array) => array.sort());

  return <any>Object.entries(sorted).sort(([lenA], [lenB]) => +lenB - +lenA);
}

export function statusToBoolean(status: Status) {
  return {
    isIdle: status === Status.IDLE,
    isError: status === Status.ERROR,
    isRunning: status === Status.RUNNING,
    isCompleted: status === Status.COMPLETED,
  } as const;
}

export function validateQuery(arg: { query: string; prevQuery?: string }) {
  const { query, prevQuery = null } = arg;

  let message: string;

  if (!query) message = "Query cannot be empty.";
  else if (query.length < 3)
    message = "Query must be at least 3 characters long.";
  else if (query.length > MAX_WORD_LEN)
    message = `Query cannot be longer than ${MAX_WORD_LEN} characters.`;
  else if (query === prevQuery) message = "Same as previous query.";
  else message = "";

  return { isValid: !message, message };
}

export async function writeClipboardText(text: string) {
  try {
    await navigator.clipboard.writeText(text);
  } catch (error) {
    alert("Could not copy text to clipboard!");
  }
}
