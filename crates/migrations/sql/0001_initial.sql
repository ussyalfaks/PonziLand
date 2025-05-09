CREATE TYPE event_type AS ENUM (
    'ponzi_land-AuctionFinishedEvent',
    'ponzi_land-LandBoughtEvent',
    'ponzi_land-LandNukedEvent',
    'ponzi_land-NewAuctionEvent',
    'ponzi_land-RemainingStakeEvent',
    'ponzi_land-AddressAuthorizedEvent',
    'ponzi_land-AddressRemovedEvent',
    'ponzi_land-VerifierUpdatedEvent'
);

CREATE TABLE event (
    id uuid NOT NULL PRIMARY KEY,
    at timestamp without time zone NOT NULL,
    event_type event_type NOT NULL
);

CREATE TABLE event_auction_finished (
    id uuid NOT NULL PRIMARY KEY,
    location smallint NOT NULL,
    start_time timestamp without time zone NOT NULL,
    buyer text NOT NULL,
    at timestamp without time zone NOT NULL,
    sold_at timestamp without time zone NOT NULL
);

CREATE TABLE event_address_authorized (
    id uuid NOT NULL PRIMARY KEY,
    at timestamp without time zone NOT NULL,
    address text NOT NULL
);

CREATE TABLE event_address_removed (
    id uuid NOT NULL PRIMARY KEY,
    at timestamp without time zone NOT NULL,
    address text NOT NULL
);

CREATE TABLE event_land_bought (
    id uuid NOT NULL PRIMARY KEY,
    location smallint NOT NULL,
    buyer text NOT NULL,
    at timestamp without time zone NOT NULL,
    price bigint NOT NULL,
    token_used text NOT NULL
);

CREATE TABLE event_new_auction (
    id uuid NOT NULL PRIMARY KEY,
    location smallint NOT NULL,
    at timestamp without time zone NOT NULL,
    starting_price bigint NOT NULL,
    floor_price bigint NOT NULL
);

CREATE TABLE event_nuked (
    id uuid NOT NULL PRIMARY KEY,
    location smallint NOT NULL,
    owner text NOT NULL,
    at timestamp without time zone NOT NULL
);

CREATE TABLE event_remaining_stake (
    id uuid NOT NULL PRIMARY KEY,
    at timestamp without time zone NOT NULL,
    location smallint NOT NULL,
    remaining_stake bigint NOT NULL
);

CREATE TABLE historical_land (
    id uuid NOT NULL PRIMARY KEY,
    at timestamp without time zone NOT NULL,
    location smallint NOT NULL,
    bought_at timestamp without time zone NOT NULL,
    owner text NOT NULL,
    sell_price bigint NOT NULL,
    token text NOT NULL,
    level smallint NOT NULL
);

CREATE TABLE historical_land_stake (
    id uuid NOT NULL PRIMARY KEY,
    at timestamp without time zone NOT NULL,
    location smallint NOT NULL,
    last_pay_time timestamp without time zone NOT NULL,
    amount bigint NOT NULL
);
