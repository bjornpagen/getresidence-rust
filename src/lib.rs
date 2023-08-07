use http::StatusCode;
use worker::*;

mod handlers;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/dubai", handlers::dubai_handler)
        .get_async("/dubai/onboarding", handlers::dubai_onboarding_handler)
        .get_async("/", handlers::redirect_dubai)
        .run(req, env)
        .await
}
