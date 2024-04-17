use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Beer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Beer::BeerId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Beer::Name).string().not_null())
                    .col(ColumnDef::new(Beer::Style).string().not_null())
                    .col(ColumnDef::new(Beer::GroupFilter).string().not_null())
                    .col(ColumnDef::new(Beer::StyleFilter).string())
                    .col(ColumnDef::new(Beer::Brewery).string().not_null())
                    .col(ColumnDef::new(Beer::Abv).float())
                    .col(ColumnDef::new(Beer::Description).string())
                    .col(ColumnDef::new(Beer::ImgUrl).string())
                    .col(ColumnDef::new(Beer::LastReview).string())
                    .col(ColumnDef::new(Beer::Location).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Beer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Beer {
    Table,
    BeerId,
    Name,
    Style,
    GroupFilter,
    StyleFilter,
    Brewery,
    Abv,
    Description,
    ImgUrl,
    LastReview,
    Location,
}
