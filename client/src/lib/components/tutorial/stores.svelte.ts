import type { Tile } from '$lib/api/tile-store.svelte';
import { toHexWithPadding } from '$lib/utils';
import data from '$lib/data.json';

export const MAP_SIZE = 16;

function createFakeTiles(): Tile[][] {
  return Array(MAP_SIZE)
    .fill(null)
    .map((_, i) =>
      Array(MAP_SIZE)
        .fill(null)
        .map((_, j) => ({
          location: toHexWithPadding(i * MAP_SIZE + j),
          type: 'grass',
          timeToNuke: 10000000,
        })),
    );
}

class TileState {
  public tilesStore: Tile[][] = $state(createFakeTiles());
  public nuke = $state(false);
  getTiles() {
    return this.tilesStore;
  }

  getNuke() {
    return this.nuke;
  }

  setNuke(nuke: boolean) {
    this.nuke = nuke;
  }

  addAuction(x: number = 8, y: number = 8): void {
    if (this.tilesStore[x] && this.tilesStore[x][y]) {
      this.tilesStore[x][y] = {
        ...this.tilesStore[x][y],
        type: 'auction',
        owner: '0x',
      };
    }
  }

  removeAuction(x: number = 8, y: number = 8): void {
    if (this.tilesStore[x] && this.tilesStore[x][y]) {
      this.tilesStore[x][y] = {
        ...this.tilesStore[x][y],
        type: 'grass',
      };
    }
  }

  buyAuction(x: number = 8, y: number = 8, tokenId: number = 0): void {
    if (this.tilesStore[x] && this.tilesStore[x][y]) {
      this.tilesStore[x][y] = {
        ...this.tilesStore[x][y],
        type: 'house',
        level: 1 as ( 1 | 2 | 3),
        token: data.availableTokens[tokenId],
      };
    }
  }

  reduceTimeToNuke(x: number, y: number): void {
    if (this.tilesStore[x] && this.tilesStore[x][y]) {
      this.tilesStore[x][y].timeToNuke = this.tilesStore[x][y].timeToNuke / 2;
    }
  }

  levelUp(x: number, y: number): void {
    console.log('qsdfkjmqdsfl',this.tilesStore[x][y]);
    if (
      this.tilesStore[x] &&
      this.tilesStore[x][y] &&
      this.tilesStore[x][y].type === 'house' &&
      'level' in this.tilesStore[x][y]
    ) {
      console.log('This is a house');
      this.tilesStore[x][y] = {
        ...this.tilesStore[x][y],
        level: Number(this.tilesStore[x][y].level) + 1 as ( 1 | 2 | 3),
      };
    }
  }
}

export const tileState = new TileState();

export function tutorialProgression() {
  let value = $state(1);

  return {
    get value() {
      return value;
    },
    increment: () => (value += 1),
    decrement: () => (value -= 1),
  };
}
