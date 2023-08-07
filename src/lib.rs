use worker::*;

mod handlers;

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/dubai", handlers::dubai_handler)
        .get_async("/", handlers::redirect_dubai)
        .run(req, env)
        .await
}
