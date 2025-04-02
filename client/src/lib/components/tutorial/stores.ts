import { writable } from 'svelte/store';
import type { Tile } from '$lib/api/tile-store.svelte';
import { toHexWithPadding } from '$lib/utils';

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

export function addAuctionToTiles(): void {
  tiles.update((currentTiles) => {
    const newTiles = [...currentTiles];

    if (newTiles[8] && newTiles[8][8]) {
      newTiles[8] = [...newTiles[8]];
      newTiles[8][8] = {
        ...newTiles[8][8],
        type: 'auction',
      };
    }

    return newTiles;
  });
}

export function removeAuctionFromTiles(): void {
  tiles.update((currentTiles) => {
    const newTiles = [...currentTiles];

    if (newTiles[8] && newTiles[8][8]) {
      newTiles[8] = [...newTiles[8]];
      newTiles[8][8] = {
        ...newTiles[8][8],
        type: 'grass',
      };
    }

    return newTiles;
  });
}

export const tiles = writable<Tile[][]>(createFakeTiles());
export const tutorialProgression = writable<number>(1);
