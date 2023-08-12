#![feature(async_fn_in_trait)]

use worker::*;

mod handlers;

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/", handlers::get_root)
        .get_async("/privacy", handlers::get_privacy)
        .get_async("/dubai", handlers::get_dubai)
        .put_async("/dubai/name", handlers::put_dubai_name)
        .put_async("/dubai/email", handlers::put_dubai_email)
        .put_async("/dubai/phone", handlers::put_dubai_phone)
        .run(req, env)
        .await
}
