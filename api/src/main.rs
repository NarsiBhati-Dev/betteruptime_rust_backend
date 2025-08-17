use poem::{Route, Server, get, handler, listener::TcpListener, post, web::Path};

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[handler]
fn create_website(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}
