use actix_files::NamedFile;
use actix_web::{
    http::Uri,
    middleware::{self, TrailingSlash},
    web, App, HttpServer,
};
use env_logger::Env;
use std::time::Duration;

async fn index() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

async fn static_file(uri: Uri) -> std::io::Result<NamedFile> {
    let path = uri.path();
    println!("{}", format!("./static{path}"));
    Ok(NamedFile::open(format!("./static{path}"))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host = 8080;
    println!("Server starting at http://localhost:{}", host);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{any_path}", web::get().to(static_file))
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .wrap(middleware::Logger::new("%s %r %a %D")) // Keep logger at end
    })
    .keep_alive(Duration::from_secs(60))
    .bind(("0.0.0.0", host))?
    .run()
    .await
}
