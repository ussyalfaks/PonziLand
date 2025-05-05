use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(HistoricalLand::Table)
                    .if_not_exists()
                    .col(pk_uuid(HistoricalLand::Id))
                    .col(date_time(HistoricalLand::At))
                    .col(small_integer(HistoricalLand::Location))
                    .col(date_time(HistoricalLand::BoughtAt))
                    .col(text(HistoricalLand::Owner))
                    .col(big_integer(HistoricalLand::SellPrice))
                    .col(text(HistoricalLand::Token))
                    .col(tiny_unsigned(HistoricalLand::Level))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(HistoricalLandStake::Table)
                    .if_not_exists()
                    .col(pk_uuid(HistoricalLandStake::Id))
                    .col(date_time(HistoricalLandStake::At))
                    .col(small_integer(HistoricalLandStake::Location))
                    .col(date_time(HistoricalLandStake::LastPayTime))
                    .col(big_integer(HistoricalLandStake::Amount))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(HistoricalLand::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum HistoricalLand {
    Table,
    Id,

    At,
    Location,
    BoughtAt,
    Owner,
    SellPrice,
    Token,
    Level,
}

#[derive(DeriveIden)]
enum HistoricalLandStake {
    Table,
    Id,

    At,

    Location,
    LastPayTime,
    Amount,
}
