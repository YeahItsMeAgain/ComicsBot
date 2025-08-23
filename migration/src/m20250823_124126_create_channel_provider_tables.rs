use crate::m20220101_000001_create_channel_table::Channel;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum ComicsProvider {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum ChannelComicsProviderSubscription {
    Table,
    ChannelId,
    ProviderId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ComicsProvider::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComicsProvider::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ComicsProvider::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create join table
        manager
            .create_table(
                Table::create()
                    .table(ChannelComicsProviderSubscription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChannelComicsProviderSubscription::ChannelId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ChannelComicsProviderSubscription::ProviderId)
                            .integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ChannelComicsProviderSubscription::ChannelId)
                            .col(ChannelComicsProviderSubscription::ProviderId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ChannelComicsProviderSubscription::Table,
                                ChannelComicsProviderSubscription::ChannelId,
                            )
                            .to(Channel::Table, Channel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ChannelComicsProviderSubscription::Table,
                                ChannelComicsProviderSubscription::ProviderId,
                            )
                            .to(ComicsProvider::Table, ComicsProvider::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ChannelComicsProviderSubscription::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(ComicsProvider::Table).to_owned())
            .await?;
        Ok(())
    }
}
