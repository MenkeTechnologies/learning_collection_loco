#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
use rand::seq::SliceRandom;
use sea_orm::{ActiveValue, EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize;
use std::process::Command;

use crate::models::_entities::learning_collections::{
    ActiveModel, Column, Entity as LearningCollections, Model,
};

pub const SHORT_CNT: usize = 20;
pub const DEFAULT_CAT: &str = "programming";
pub const DB_NAME: &str = "root";

fn dump_cmd() -> String {
    format!("mysqldump --extended-insert=FALSE {DB_NAME}")
}

#[derive(Debug, Deserialize)]
pub struct LearningQuery {
    pub learning: String,
}

async fn all_ordered(ctx: &AppContext) -> Result<Vec<Model>> {
    let mut rows = LearningCollections::find().all(&ctx.db).await?;
    rows.sort_by_key(|m| m.id);
    Ok(rows)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Query(q): Query<LearningQuery>,
) -> Result<Response> {
    let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    let item = ActiveModel {
        learning: ActiveValue::set(q.learning),
        category: ActiveValue::set(DEFAULT_CAT.to_string()),
        date_added: ActiveValue::set(now),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await?;
    format::json(item)
}

#[debug_handler]
pub async fn filter_learn(
    State(ctx): State<AppContext>,
    Query(q): Query<LearningQuery>,
) -> Result<Response> {
    let pattern = format!("%{}%", q.learning);
    let rows = LearningCollections::find()
        .filter(Column::Learning.like(pattern))
        .all(&ctx.db)
        .await?;
    let items: Vec<String> = rows.into_iter().map(|m| m.learning).collect();
    format::json(items)
}

#[debug_handler]
pub async fn recents_default(State(ctx): State<AppContext>) -> Result<Response> {
    let items: Vec<String> = all_ordered(&ctx)
        .await?
        .into_iter()
        .rev()
        .take(SHORT_CNT)
        .map(|m| m.learning)
        .collect();
    format::json(items)
}

#[debug_handler]
pub async fn recents_count(
    State(ctx): State<AppContext>,
    Path(count): Path<usize>,
) -> Result<Response> {
    let items: Vec<String> = all_ordered(&ctx)
        .await?
        .into_iter()
        .rev()
        .take(count)
        .map(|m| m.learning)
        .collect();
    format::json(items)
}

#[debug_handler]
pub async fn recent_count(
    State(ctx): State<AppContext>,
    Path(count): Path<usize>,
) -> Result<Response> {
    let items: Vec<Model> = all_ordered(&ctx).await?.into_iter().rev().take(count).collect();
    format::json(items)
}

#[debug_handler]
pub async fn dump() -> Result<Response> {
    tracing::debug!("mysqldump to response body");
    let cmd = dump_cmd();
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let out = Command::new(parts[0])
        .args(&parts[1..])
        .output()
        .map_err(|e| Error::Message(format!("mysqldump failed: {e}")))?;
    let body = String::from_utf8_lossy(&out.stdout).to_string();
    format::text(&body)
}

#[debug_handler]
pub async fn random_one(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = LearningCollections::find().all(&ctx.db).await?;
    let mut rng = rand::thread_rng();
    let pick = rows
        .choose(&mut rng)
        .cloned()
        .ok_or_else(|| Error::NotFound)?;
    format::json(pick)
}

#[debug_handler]
pub async fn randoms_one(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = LearningCollections::find().all(&ctx.db).await?;
    let mut rng = rand::thread_rng();
    let pick = rows
        .choose(&mut rng)
        .map(|m| m.learning.clone())
        .ok_or_else(|| Error::NotFound)?;
    format::json(pick)
}

#[debug_handler]
pub async fn randoms_count(
    State(ctx): State<AppContext>,
    Path(count): Path<usize>,
) -> Result<Response> {
    let mut rows = LearningCollections::find().all(&ctx.db).await?;
    let mut rng = rand::thread_rng();
    rows.shuffle(&mut rng);
    let items: Vec<String> = rows.into_iter().take(count).map(|m| m.learning).collect();
    format::json(items)
}

#[debug_handler]
pub async fn random_count(
    State(ctx): State<AppContext>,
    Path(count): Path<usize>,
) -> Result<Response> {
    let mut rows = LearningCollections::find().all(&ctx.db).await?;
    let mut rng = rand::thread_rng();
    rows.shuffle(&mut rng);
    let items: Vec<Model> = rows.into_iter().take(count).collect();
    format::json(items)
}

// Spring Data REST-style auto CRUD at /learning
#[debug_handler]
pub async fn list_all(State(ctx): State<AppContext>) -> Result<Response> {
    let rows = LearningCollections::find().all(&ctx.db).await?;
    format::json(rows)
}

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    let item = LearningCollections::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    format::json(item)
}

#[debug_handler]
pub async fn delete_one(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Response> {
    LearningCollections::delete_by_id(id).exec(&ctx.db).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/add", get(add))
        .add("/filter", get(filter_learn))
        .add("/recents", get(recents_default))
        .add("/recents/{count}", get(recents_count))
        .add("/recent/{count}", get(recent_count))
        .add("/dump", get(dump))
        .add("/random", get(random_one))
        .add("/randoms", get(randoms_one))
        .add("/randoms/{count}", get(randoms_count))
        .add("/random/{count}", get(random_count))
        .add("/learning", get(list_all))
        .add("/learning/{id}", get(get_one))
        .add("/learning/{id}", delete(delete_one))
}
