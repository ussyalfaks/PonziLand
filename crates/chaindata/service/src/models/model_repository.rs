use sqlx::{postgres::PgRow, query_builder::Separated, FromRow, QueryBuilder};

use super::event::EventId;
use sqlx::Error;

#[async_trait::async_trait]
pub trait EventModelRepository: Sized + for<'r> FromRow<'r, PgRow> + Unpin + Send + Sync {
    const TABLE_NAME: &'static str;

    async fn get_by_id<'e, Conn>(db: Conn, id: EventId) -> Result<Option<Self>, Error>
    where
        Conn: 'e + sqlx::Executor<'e, Database = sqlx::Postgres>,
    {
        // Build query builder
        let mut query = QueryBuilder::new("SELECT ");
        Self::push_parameters(&mut query);

        query
            .push(" FROM ")
            .push(Self::TABLE_NAME)
            .push(" WHERE id = ?")
            .push_bind(id);

        query.build_query_as().fetch_optional(db).await
    }

    fn push_parameters<'a>(query: &mut QueryBuilder<'a, sqlx::Postgres>);
    fn push_tuple<'args>(args: Separated<'_, 'args, sqlx::Postgres, &'static str>, model: &Self);

    async fn save<'e, Conn>(db: Conn, model: &Self) -> Result<(), Error>
    where
        Conn: 'e + sqlx::Executor<'e, Database = sqlx::Postgres>,
    {
        // Build query builder
        let mut query = QueryBuilder::new("INSERT INTO ");
        query.push(Self::TABLE_NAME).push(" (");
        Self::push_parameters(&mut query);
        query.push(") ");

        query.push_values(std::iter::once(model), Self::push_tuple);

        query.build().execute(db).await?;
        Ok(())
    }
}
