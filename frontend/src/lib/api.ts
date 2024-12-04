import { type API_State, Status, type API_Response } from "./interface";
import { writable, type Subscriber, type Readable } from "svelte/store";
import { sortWordsBasedOnLength } from "./util";
import { BASE_URL } from "../config";

export class API {
  #controller = new AbortController();

  readonly #stateStore: Readable<API_State>;
  readonly #updateStore: Subscriber<API_State>;

  #state: API_State = Object.freeze({ status: Status.IDLE });

  constructor() {
    const { set, subscribe } = writable<API_State>(this.#state);

    this.#stateStore = { subscribe };
    this.#updateStore = <any>set;
  }

  get state() {
    return this.#stateStore;
  }

  #setState(state: API_State) {
    this.#state = state;

    this.#updateStore(this.#state);
  }

  get isRunning() {
    return this.#state.status === Status.RUNNING;
  }

  get isIdle() {
    return this.#state.status === Status.IDLE;
  }

  async search(query: string) {
    if (this.isRunning) return;

    {
      const hasAlreadyFetchedResult =
        this.#state.status === Status.COMPLETED &&
        this.#state.data.query === query;

      if (hasAlreadyFetchedResult) return;
    }

    this.#setState({ status: Status.RUNNING });

    try {
      const rawResponse = await fetch(makeUrl(query), {
        signal: this.#controller.signal,
      });

      const response = (await rawResponse.json()) as API_Response;

      if (!response.possibleWords?.length)
        return this.#setState({
          status: Status.ERROR,
          message: `Sorry, no words found :(`,
        });

      const sortedWords = sortWordsBasedOnLength(response.possibleWords);

      this.#setState({
        status: Status.COMPLETED,
        data: { ...response, sortedWords, topMatch: sortedWords[0][1][0] },
      });
    } catch (ex) {
      this.#setState({
        status: Status.ERROR,
        message: ex?.message || "Server error or could not connect :(",
      });
    }
  }

  stopOrReset() {
    if (this.isIdle) return;

    if (this.isRunning) {
      this.#controller.abort();
      this.#controller = new AbortController();
    }
    this.#setState({ status: Status.IDLE });
  }
}

function makeUrl(word: string) {
  return encodeURI(`${BASE_URL}/unscramble?word=${word}`);
}
