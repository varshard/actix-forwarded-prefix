use actix_web::{web, App, get, HttpRequest, HttpServer};
use actix_forward_prefix::{ForwardPrefix};

#[get("/banana")]
async fn banana(req: HttpRequest) -> &'static str {
	"Hello world!\r\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.wrap(ForwardPrefix)
			.route(
				"/index.html",
				web::get().to(|| async { "Hello, middleware!" }),
			)
			.service(banana)
	})
		.bind(("127.0.0.1", 8090))?
		.workers(1)
		.run()
		.await
}