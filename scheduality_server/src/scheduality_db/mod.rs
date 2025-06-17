use sqlx::{pool::Pool, Database, Postgres};

#[derive(Debug)]
pub struct DbConnectionError(sqlx::Error);
impl std::fmt::Display for DbConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:#?}", self).as_str())
    }
}

impl std::error::Error for DbConnectionError {}

pub struct SchedualityDb<DB>
where
    DB: Database,
{
    connections_pool: Pool<DB>,
}

impl<DB> Clone for SchedualityDb<DB>
where
    DB: Database,
{
    fn clone(&self) -> Self {
        Self {
            connections_pool: self.connections_pool.clone(),
        }
    }
}

impl SchedualityDb<Postgres> {
    pub async fn connect(connection_string: &str) -> Result<Self, DbConnectionError> {
        match sqlx::postgres::PgPool::connect(connection_string).await {
            Ok(connections_pool) => Ok(Self { connections_pool }),
            Err(e) => Err(DbConnectionError(e)),
        }
    }

    pub async fn drop_tables(&self) -> Result<(), sqlx::Error> {
        use std::fs;

        let drop_query = fs::read_to_string("./schema/drop.sql").unwrap();
        sqlx::raw_sql(&drop_query)
            .execute(&self.connections_pool)
            .await?;

        Ok(())
    }

    pub async fn setup(&self) -> Result<(), sqlx::Error> {
        use std::fs;

        let tables_schema = fs::read_to_string("./schema/tables.sql")?;
        let tables_future = sqlx::raw_sql(&tables_schema).execute(&self.connections_pool);
        let functions_schema = fs::read_to_string("./schema/functions.sql")?;
        let views_schema = fs::read_to_string("./schema/views.sql")?;
        tables_future.await?;

        let functions_future = sqlx::raw_sql(&functions_schema).execute(&self.connections_pool);
        let views_future = sqlx::raw_sql(&views_schema).execute(&self.connections_pool);

        functions_future.await?;
        views_future.await?;

        Ok(())
    }
}
