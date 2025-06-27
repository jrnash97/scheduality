use sqlx::{pool::Pool, Postgres};

pub type SchedualityDb<DB> = Pool<DB>;

pub trait SchedualityDbExt {
    #[allow(async_fn_in_trait)]
    async fn drop_tables(&self) -> Result<(), sqlx::Error>;
    #[allow(async_fn_in_trait)]
    async fn setup(&self) -> Result<(), sqlx::Error>;
}

impl SchedualityDbExt for SchedualityDb<Postgres> {
    async fn drop_tables(&self) -> Result<(), sqlx::Error> {
        use std::fs;

        let drop_query = fs::read_to_string("./schema/drop.sql").unwrap();
        sqlx::raw_sql(&drop_query).execute(self).await?;

        Ok(())
    }

    async fn setup(&self) -> Result<(), sqlx::Error> {
        use std::fs;

        let tables_schema = fs::read_to_string("./schema/tables.sql")?;
        let functions_schema = fs::read_to_string("./schema/functions.sql")?;
        let views_schema = fs::read_to_string("./schema/views.sql")?;
        sqlx::raw_sql(&tables_schema).execute(self).await?;
        sqlx::raw_sql(&functions_schema).execute(self).await?;
        sqlx::raw_sql(&views_schema).execute(self).await?;

        Ok(())
    }
}
