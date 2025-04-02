import { writable } from 'svelte/store';
import type { Tile } from '$lib/api/tile-store.svelte';
import { toHexWithPadding } from '$lib/utils';

export const tutorialProgression = writable<number>(1);

const MAP_SIZE = 16;

export function createFakeTiles(): Tile[][] {
  return Array(MAP_SIZE)
    .fill(null)
    .map((_, i) =>
      Array(MAP_SIZE)
        .fill(null)
        .map((_, j) => ({
          location: toHexWithPadding(i * MAP_SIZE + j),
          type: 'grass',
        })),
    );
}

export function createEmptyTiles(): Tile[][] {
  return Array(MAP_SIZE)
    .fill(null)
    .map((_, i) =>
      Array(MAP_SIZE)
        .fill(null)
        .map((_, j) => ({
          location: toHexWithPadding(i * MAP_SIZE + j),
          type: 'empty',
          block_date_bought: null,
          owner: null,
          sell_price: 0,
          token_used: null,
        })),
    );
}

export const tiles = writable<Tile[][]>(createFakeTiles());
