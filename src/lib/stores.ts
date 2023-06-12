import { writable } from "svelte/store";

export const query_store = writable<string[]>(new Array<string>());
export const terms_store = writable<number[]>(new Array<number>());
