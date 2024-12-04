export const MAX_WORD_LEN = 31;
export const BASE_URL =
  import.meta.env.MODE === "development"
    ? "http://localhost:8080"
    : window.origin;
