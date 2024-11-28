export interface API_Response {
  query: string;
  searchTime: number;
  possibleWords: string[];
}

export interface SearchResult {
  query: string;
  topMatch: string;
  searchTime: number;
  possibleWords: string[];
  sortedWords: [number, string[]][];
}

export const enum Status {
  IDLE = "idle",
  ERROR = "error",
  RUNNING = "running",
  COMPLETED = "completed",
}

export type API_State =
  | { status: Status.IDLE | Status.RUNNING }
  | { status: Status.ERROR; message: string }
  | { status: Status.COMPLETED; data: SearchResult };
