import { writable } from 'svelte/store';

export const tutorialProgression = writable<number>(1);
