use chrono::{DateTime, Utc};
use diesel::{
    ExpressionMethods, Insertable, PgTextExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::kill::Kill;
use crate::infra::errors::{adapt_infra_error, InfraError};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = kills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KillDb {
    pub id: Uuid,
    pub killer: String,
    pub killed: String,
    pub range: f32,
    pub gun: String,
    pub date: DateTime<Utc>,
    pub server: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = kills)]
pub struct NewKillDb {
    pub killer: String,
    pub killed: String,
    pub range: f32,
    pub gun: String,
    pub date: DateTime<Utc>,
    pub server: String,
}

#[derive(Deserialize)]
pub struct PostsFilter {
    published: Option<bool>,
    title_contains: Option<String>,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_post: NewPostDb,
) -> Result<Kill, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(posts::table)
                .values(new_post)
                .returning(PostDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_post_db_to_post(res))
}

pub async fn get(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<Kill, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            posts::table
                .filter(posts::id.eq(id))
                .select(PostDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_post_db_to_post(res))
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: PostsFilter,
) -> Result<Vec<Kill>, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            let mut query = posts::table.into_boxed::<diesel::pg::Pg>();

            if let Some(published) = filter.published {
                query = query.filter(posts::published.eq(published));
            }

            if let Some(title_contains) = filter.title_contains {
                query = query.filter(posts::title.ilike(format!("%{}%", title_contains)));
            }

            query.select(PostDb::as_select()).load::<PostDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    let posts: Vec<Kill> = res
        .into_iter()
        .map(|post_db| adapt_post_db_to_post(post_db))
        .collect();

    Ok(posts)
}

fn adapt_post_db_to_post(post_db: PostDb) -> Kill {
    Kill {
        id: post_db.id,
        title: post_db.title,
        body: post_db.body,
        published: post_db.published,
    }
}
