import { writable } from 'svelte/store';

export const tileHUD = writable<{
    location: number;
    owner: string | null;   
} | null>(null);
