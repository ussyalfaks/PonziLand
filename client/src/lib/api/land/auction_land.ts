import type { Token } from '$lib/interfaces';
import type { Land } from '$lib/models.gen';
import { getTokenInfo } from '$lib/utils';
import { CurrencyAmount } from '$lib/utils/CurrencyAmount';
import { fromDojoLevel, type Level } from '$lib/utils/level';
import type { BigNumberish } from 'starknet';
import { BaseLand } from '.';
import { locationEquals, toLocation } from './location';

export class AuctionLand extends BaseLand {
  private _owner!: string;
  private _boughtAt!: Date;
  private _sellPrice!: CurrencyAmount;
  private _token!: Token;
  private _level!: Level;
  private _stakeAmount!: CurrencyAmount;
  private _lastPayTime!: Date;
  private _block_date_bought: BigNumberish;
  private _sell_price: BigNumberish;
  private _token_used: string;

  constructor(land: Land) {
    super('auction', toLocation(land.location)!);
    this._block_date_bought = land.block_date_bought;
    this._sell_price = land.sell_price;
    this._token_used = land.token_used;
    this.update(land);
  }

  public update(land: Land) {
    // Assert that the location is the same
    if (!locationEquals(toLocation(land.location)!, this.location)) {
      console.error(
        'Wrong location!',
        land,
        toLocation(land.location),
        this.location,
      );
    }

    this._boughtAt = new Date(Number(land.block_date_bought));
    this._owner = land.owner;
    this._level = fromDojoLevel(land.level);

    this._token = getTokenInfo(land.token_used)!;
    this._sellPrice = CurrencyAmount.fromUnscaled(land.sell_price, this._token);
  }

  static is(land: BaseLand): land is AuctionLand {
    return land.type === 'auction';
  }

  //region Accessors
  public get owner(): string {
    return this._owner;
  }

  public get level(): Level {
    return this._level;
  }

  public get boughtAt(): Date {
    return this._boughtAt;
  }

  public get sellPrice(): CurrencyAmount {
    return this._sellPrice;
  }

  public get token(): Token {
    return this._token;
  }

  public get stakeAmount(): CurrencyAmount {
    return this._stakeAmount;
  }

  public get lastPayTime(): Date {
    return this._lastPayTime;
  }

  public get block_date_bought(): BigNumberish {
    return this._block_date_bought;
  }

  public get sell_price(): BigNumberish {
    return this._sell_price;
  }

  public get token_used(): string {
    return this._token_used;
  }

  public get tokenUsed(): string {
    return this._token_used;
  }

  public get tokenAddress(): string {
    return this._token.address;
  }
  //endregion
}
