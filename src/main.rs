use actix_web::{web, App, HttpResponse, HttpServer};
use meilisearch_sdk::Client;

async fn search() -> HttpResponse {
    let client = Client::new(
        "https://ms-2f5ad7ce56a7-1512.sfo.meilisearch.io/",
        "6287312fd043d3fca95136cd40483a26154d37dc99aa2e79417f88794a80cd1c",
    );
    let index = client.index("movies-en-US");
    let results = index.search().execute::<serde_json::Value>().await.unwrap();
    let results = results
        .hits
        .into_iter()
        .map(|res| res.result)
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(search))))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
