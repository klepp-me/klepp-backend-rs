use axum::extract::{Query, State};
use crate::http::types::Result;
use crate::http::extractor::ApiContext;
use axum::routing::get;
use axum::{Json, Router};
use crate::http::Error;
use crate::http::Error::Anyhow;


pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route(
            "/api/videos",
            get(list_videos),
        )
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct ListVideosQuery {
    // todo: Filter on multiple tags
    tag: Option<String>,

    // todo: I should probably paginate using a value of an indexed column, such as `created_at`
    //  including the `limit`. Offset will do for now.
    limit: Option<i64>,
    offset: Option<i64>,
}


#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Tag {
    name: String
}



#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    thumbnail_uri: Option<String>
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Video {
    user: String,
    tags: Vec<Tag>,
    thumbnail_uri: Option<String>,
    likes: Vec<User>,
    path: String,
    display_name: String,
    hidden: bool,
    uri: String,
    // todo: parse
    // uploaded_at: Timestamp,
    // expire_at: Option<Timestamp>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleVideosBody {
    response: Vec<Video>,
    total_count: usize,
}

fn try_thing() -> Result<(), anyhow::Error> {
    // https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs#L32-L34
    anyhow::bail!("it failed!")
}


pub(in crate::http) async fn list_videos(
    // user: UserOrUnknown,
    _ctx: State<ApiContext>,
    _query: Query<ListVideosQuery>,
) -> Result<Json<MultipleVideosBody>> {
    // https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about
    try_thing()?;
    Ok(Json(MultipleVideosBody {
        response: vec![Video {
                user: "Hello".to_string(),
                tags: vec![Tag{name: "Hehe".to_string()}],
                thumbnail_uri: None,
                likes: vec![User {name: "Jonas".to_string(), thumbnail_uri: None}],
                path: "Hello".to_string(),
                display_name: "Some name".to_string(),
                hidden: true,
                uri: "String".to_string(),
        }],
        total_count: 1,
    }))
}