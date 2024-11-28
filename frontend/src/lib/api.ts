import { type API_State, Status } from "./interface";
import { writable, type Subscriber, type Readable } from "svelte/store";

export class API {
  #stateStore: Readable<API_State>;
  #updateStore: Subscriber<API_State>;

  #state: API_State = Object.freeze({ status: Status.IDLE });
  #timerId: any;

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

  search(query: string) {
    console.log("search:", query);

    if (this.isRunning) return;

    this.#setState({ status: Status.RUNNING });

    this.#timerId = setTimeout(() => {
      if (Math.random() > 0.5) {
        this.#setState({
          status: Status.COMPLETED,
          data: {
            possibleWords: ["hello", "there", "how", "are", "you"],
            query: "elloh",
            searchTime: 23,
            sortedWords: [
              [5, ["hello", "there"]],
              [3, ["how", "are", "you"]],
            ],
            topMatch: "hello",
          },
        });

        return;
      }

      this.#setState({
        status: Status.ERROR,
        message: `Could not connect to server. [id: ${Math.random()}]`,
      });
    }, Math.random() * 3000 + 1000);
  }

  stopOrReset() {
    if (this.isIdle) return;

    clearTimeout(this.#timerId);
    this.#setState({ status: Status.IDLE });
  }
}
