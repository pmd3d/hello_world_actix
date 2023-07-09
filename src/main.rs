use actix_web::{ App, HttpServer, HttpResponse, HttpRequest, handler };

// pub struct greet;
// impl ::actix_web::dev::HttpServiceFactory for greet {
//     fn register(self, __config: &mut actix_web::dev::AppService) {
//         async fn greet(req: HttpRequest) -> impl Responder {
//             let name = req.match_info().get("name").unwrap_or("World");
//             format!("Hello, {}!", &name)
//         }
//         let __resource = ::actix_web::Resource::new("/")
//             .name("greet")
//             .guard(::actix_web::guard::Get())
//             .to(greet);
//         ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
//     }
// }

#[handler("/")]
async fn example(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(example)        
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// #[get("/")]
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello, {}!", &name)
// }
//
// cargo install cargo-expand
// cargo +nightly expand

