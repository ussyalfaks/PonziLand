import type { LandYieldInfo, Token } from '$lib/interfaces';
import type { Auction, Land, LandStake } from '$lib/models.gen';
import { coordinatesToLocation, toHexWithPadding } from '$lib/utils';
import type { CurrencyAmount } from '$lib/utils/CurrencyAmount';
import { type Level } from '$lib/utils/level';

import type { Neighbors } from '../neighbors';
import { type Location } from './location';

export type LandType = 'empty' | 'auction' | 'building';

export type TransactionResult = Promise<
  | {
      transaction_hash: string;
    }
  | undefined
>;

export type PendingTax = {
  amount: CurrencyAmount;
  tokenAddress: string;
};

export type NextClaimInformation = {
  amount: CurrencyAmount;
  tokenAddress: string;
  landLocation: string;
  canBeNuked: boolean;
};

export type LevelInfo = {
  canLevelUp: boolean;
  expectedLevel: Level;
  timeSinceLastLevelUp: number;
  levelUpTime: number;
};

export type LandWithStake = Land & LandStake;
export type LandAuction = Land & Auction;

export type LandSetup = {
  tokenForSaleAddress: string;
  salePrice: CurrencyAmount;
  amountToStake: CurrencyAmount;
  tokenAddress: string;
  currentPrice: CurrencyAmount | null;
};

export type LandWithMeta = Omit<
  Land | LandWithStake | LandAuction,
  'location' | 'level'
> & {
  location: string;
  // Type conversions
  stakeAmount: CurrencyAmount;
  lastPayTime: number;
  sellPrice: CurrencyAmount;

  type: 'auction' | 'house' | 'grass';
  owner: string;

  level: Level;

  tokenUsed: string | null;
  tokenAddress: string | null;

  token?: Token;
};

export type LandWithActions = LandWithMeta & {
  wait(): Promise<void>;
  increaseStake(amount: CurrencyAmount): TransactionResult;
  increasePrice(amount: CurrencyAmount): TransactionResult;
  claim(): TransactionResult;
  getPendingTaxes(): Promise<PendingTax[] | undefined>;
  getNextClaim(): Promise<NextClaimInformation[] | undefined>;
  getNukable(): Promise<number | undefined>;
  getCurrentAuctionPrice(): Promise<CurrencyAmount | undefined>;
  getYieldInfo(): Promise<LandYieldInfo | undefined>;
  getEstimatedNukeTime(): number | undefined;
  getNeighbors(): Neighbors;
  levelUp(): TransactionResult;
  getLevelInfo(): LevelInfo;
};

export abstract class BaseLand {
  private _type: LandType;
  public readonly location: Location;
  public readonly locationString: string;

  constructor(type: LandType, location: Location) {
    this._type = type;
    this.location = location;
    this.locationString = toHexWithPadding(coordinatesToLocation(location));
  }

  get type() {
    return this._type;
  }
}

export class EmptyLand extends BaseLand {
  constructor(location: Location) {
    super('empty', location);
  }

  static is(land: BaseLand): land is EmptyLand {
    return land.type === 'empty';
  }
}
