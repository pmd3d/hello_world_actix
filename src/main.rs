use actix_web::{ App, HttpServer, HttpResponse, Responder, web, guard::{self, GuardContext} };

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
//#[scope("/scope")]
//async fn scope_service() -> () {
//    actix_web::web::scope("/scope")
//        .route("/test", actix_web::web::get().to(test))
//       .route("/test_async", actix_web::web::get().to(test_async))
//}
/*
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(scope_service())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
 */
 /*

fn test_guard(_req: &GuardContext) -> bool {
    true
}
 
fn health_guard(_req: &GuardContext) -> bool {
    false
}
 
async fn test() -> HttpResponse {
     HttpResponse::Ok().body("Test endpoint")
 }
 
 async fn healthcheck() -> HttpResponse {
     HttpResponse::Ok().body("Healthcheck endpoint")
 }
 
 async fn default_route() -> HttpResponse {
     HttpResponse::NotFound().body("Route not found in /scope")
 }
 
 // Scope initialization function for "/scope"
 fn scope_config(cfg: &mut web::ServiceConfig) {
     cfg.service(
         web::resource("/test")
             .route(web::get().to(test).guard(guard::fn_guard(test_guard))));
     cfg.service(
         web::resource("/healthcheck")
             .route(web::get().to(healthcheck).guard(guard::fn_guard(health_guard))));
     cfg.default_service(
         web::route().to(default_route));
 }
*/
/*
            web::scope("/scope")
                .configure(scope_config)

*/
//async fn test2() -> impl actix_web::Responder {
//    actix_web::HttpResponse::Ok().finish()
//}
#[actix_web::scope("/works")]
const mod_inner: () = {    
    #[actix_web::get("/test")]
    #[doc("This is a test function")]
    pub async fn test() -> impl actix_web::Responder {
        mod_inner_scope::test2()
    }

    pub fn test2() -> impl actix_web::Responder {
        actix_web::HttpResponse::Ok().finish()
    }
};

/*
#[allow(non_camel_case_types, missing_docs)] 
pub struct test;
impl::actix_web::dev::HttpServiceFactory for test {
    fn register(self, __config : & mut actix_web :: dev :: AppService) {
        pub async fn test() -> impl Responder { 
            HttpResponse :: Ok().finish() 
        } 
        let __resource = ::actix_web::Resource::new("/test").name("test").guard(::actix_web::guard::Get()).to(test); 
        let __scope = ::actix_web::web::scope("/works").service(__resource);
        ::actix_web::dev::HttpServiceFactory::register(__scope, __config);
    }
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
     HttpServer::new(|| {
        App::new().service(mod_inner)
    })
    .bind("127.0.0.1:8080")?
    .run()
   .await;
   Ok(())
}
/*
#[actix_web::handler("/")]
async fn example() -> impl Responder {
    HttpResponse::Ok().finish()
}
*/

// #[get("/")]
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello, {}!", &name)
// }
//
// cargo install cargo-expand
// cargo +nightly expand

