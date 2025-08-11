use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().primary_key())
                    .col(ColumnDef::new(Users::Username).string().unique_key())
                    .col(ColumnDef::new(Users::Email).string().unique_key())
                    .col(ColumnDef::new(Users::Idp).string().not_null())
                    .col(ColumnDef::new(Users::IdpUid).string().not_null())
                    .col(
                        ColumnDef::new(Users::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_idp_uid_unique")
                    .table(Users::Table)
                    .col(Users::Idp)
                    .col(Users::IdpUid)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    Idp,
    IdpUid,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
