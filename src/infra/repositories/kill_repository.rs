use chrono::{ NaiveDateTime};
use diesel::{
    ExpressionMethods, Insertable, PgTextExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::infra::db::schema::kills;

use crate::domain::models::kill::Kill;
use crate::infra::errors::{adapt_infra_error, InfraError};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = kills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KillDb {
    pub id: Uuid,
    pub killer: String,
    pub killed: String,
    pub kill_date: NaiveDateTime
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = kills)]
pub struct NewKillDb {
    pub killer: String,
    pub killed: String,
    pub kill_date: NaiveDateTime
}

#[derive(Deserialize)]
pub struct KillsFilter {
    killer: Option<String>,
    killed: Option<String>,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_kill: NewKillDb,
) -> Result<Kill, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(kills::table)
                .values(new_kill)
                .returning(KillDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_kill_db_to_kill(res))
}

pub async fn get(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<Kill, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            kills::table
                .filter(kills::id.eq(id))
                .select(KillDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_kill_db_to_kill(res))
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: KillsFilter,
) -> Result<Vec<Kill>, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            let mut query = kills::table.into_boxed::<diesel::pg::Pg>();

            if let Some(killer) = filter.killer {
                query = query.filter(kills::killer.eq(killer));
            }

            if let Some(title_contains) = filter.killed {
                query = query.filter(kills::killer.ilike(format!("%{}%", title_contains)));
            }

            query.select(KillDb::as_select()).load::<KillDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    let kills: Vec<Kill> = res
        .into_iter()
        .map(|post_db| adapt_kill_db_to_kill(post_db))
        .collect();

    Ok(kills)
}

fn adapt_kill_db_to_kill(kill_db: KillDb) -> Kill {
    Kill {
        id: kill_db.id,
        killer: kill_db.killer,
        killed: kill_db.killed,
        kill_date: kill_db.kill_date
    }
}
