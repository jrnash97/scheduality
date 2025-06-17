use sqlx::{pool::Pool, Postgres};

pub type SchedualityDb<DB> = Pool<DB>;

pub trait ExtSchedualityDb {
    #[allow(async_fn_in_trait)]
    async fn drop_tables(&self) -> Result<(), sqlx::Error>;
    #[allow(async_fn_in_trait)]
    async fn setup(&self) -> Result<(), sqlx::Error>;
}

impl ExtSchedualityDb for SchedualityDb<Postgres> {
    async fn drop_tables(&self) -> Result<(), sqlx::Error> {
        use std::fs;

        let drop_query = fs::read_to_string("./schema/drop.sql").unwrap();
        sqlx::raw_sql(&drop_query).execute(self).await?;

        Ok(())
    }

    async fn setup(&self) -> Result<(), sqlx::Error> {
        use std::fs;

        let tables_schema = fs::read_to_string("./schema/tables.sql")?;
        let tables_future = sqlx::raw_sql(&tables_schema).execute(self);
        let functions_schema = fs::read_to_string("./schema/functions.sql")?;
        let views_schema = fs::read_to_string("./schema/views.sql")?;
        tables_future.await?;

        let functions_future = sqlx::raw_sql(&functions_schema).execute(self);
        let views_future = sqlx::raw_sql(&views_schema).execute(self);

        functions_future.await?;
        views_future.await?;

        Ok(())
    }
}
