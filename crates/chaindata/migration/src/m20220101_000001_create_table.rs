use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum EventAuctionFinished {
    Table,
    Id,

    At,

    Location,
    StartTime,

    Buyer,
    SoldAt,
}

#[derive(DeriveIden)]
enum EventLandBought {
    Table,
    Id,

    At,

    Location,

    Buyer,
    Price,
    TokenUsed,
}

#[derive(DeriveIden)]
enum EventNewAuction {
    Table,
    Id,

    At,

    Location,

    StartingPrice,
    FloorPrice,
}

#[derive(DeriveIden)]
enum EventNuked {
    Table,
    Id,

    At,

    Location,
    Owner,
}
#[derive(DeriveIden)]
enum EventRemainingStake {
    Table,
    Id,

    At,

    Location,
    RemainingStake,
}

#[derive(DeriveIden)]
enum EventAddressAuthorized {
    Table,
    Id,

    At,

    Address,
}

#[derive(DeriveIden)]
enum EventAddressRemoved {
    Table,
    Id,

    At,

    Address,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // EventAuctionFinished
        manager
            .create_table(
                Table::create()
                    .table(EventAuctionFinished::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventAuctionFinished::Id))
                    .col(small_unsigned(EventAuctionFinished::Location))
                    .col(date_time(EventAuctionFinished::StartTime))
                    .col(text(EventAuctionFinished::Buyer))
                    .col(date_time(EventAuctionFinished::At))
                    .col(date_time(EventAuctionFinished::SoldAt))
                    .to_owned(),
            )
            .await?;
        // EventLandBought
        manager
            .create_table(
                Table::create()
                    .table(EventLandBought::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventLandBought::Id))
                    .col(small_unsigned(EventLandBought::Location))
                    .col(text(EventLandBought::Buyer))
                    .col(date_time(EventLandBought::At))
                    .col(big_integer(EventLandBought::Price))
                    .col(text(EventLandBought::TokenUsed))
                    .to_owned(),
            )
            .await?;
        // EventNewAuction
        manager
            .create_table(
                Table::create()
                    .table(EventNewAuction::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventNewAuction::Id))
                    .col(small_unsigned(EventNewAuction::Location))
                    .col(date_time(EventNewAuction::At))
                    .col(big_integer(EventNewAuction::StartingPrice))
                    .col(big_integer(EventNewAuction::FloorPrice))
                    .to_owned(),
            )
            .await?;
        // EventNuked
        manager
            .create_table(
                Table::create()
                    .table(EventNuked::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventNuked::Id))
                    .col(small_unsigned(EventNuked::Location))
                    .col(text(EventNuked::Owner))
                    .col(date_time(EventNuked::At))
                    .to_owned(),
            )
            .await?;
        // EventRemainingStake
        manager
            .create_table(
                Table::create()
                    .table(EventRemainingStake::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventRemainingStake::Id))
                    .col(date_time(EventRemainingStake::At))
                    .col(small_unsigned(EventRemainingStake::Location))
                    .col(big_integer(EventRemainingStake::RemainingStake))
                    .to_owned(),
            )
            .await?;
        // Address Authorized
        manager
            .create_table(
                Table::create()
                    .table(EventAddressAuthorized::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventAddressAuthorized::Id))
                    .col(date_time(EventAddressAuthorized::At))
                    .col(text(EventAddressAuthorized::Address))
                    .to_owned(),
            )
            .await?;
        // Address Removed
        manager
            .create_table(
                Table::create()
                    .table(EventAddressRemoved::Table)
                    .if_not_exists()
                    .col(pk_uuid(EventAddressRemoved::Id))
                    .col(date_time(EventAddressRemoved::At))
                    .col(text(EventAddressRemoved::Address))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EventAuctionFinished::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EventLandBought::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EventNewAuction::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EventNuked::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EventRemainingStake::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(EventAddressAuthorized::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(EventAddressRemoved::Table).to_owned())
            .await?;

        Ok(())
    }
}
