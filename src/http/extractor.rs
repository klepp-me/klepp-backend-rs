use crate::config::Config;
use sqlx::PgPool;
use std::{
    sync::Arc,
};


// `Extractors` are kind of like `Depends` in FastAPI, or even more similar to `context` in `arq`:
//     async def create_video(
//         video: CreateVideo,
//         db: asyncpg.pool.Pool = Depends(get_database)
//     ) -> Video:
// In rust, this would look something like this:
//     async fn create_video(
//         ctx: State<ApiContext>,
//         Json(video): Json<CreateVideo>,
//     ) -> Result<Json<Video>, ApiError> {
//         let db = &ctx.db
// In other words, we can access the state by adding `State<ApiContext>` to our API function.
#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct ApiContext {
    pub config: Arc<Config>, // All config will be stored here, easily accessible
    pub db: PgPool, // Use this to interact with the database
}
