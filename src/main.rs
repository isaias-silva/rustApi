use actix_web::*;
mod routes;
use routes::ping::*;
use routes::index::*;
#[actix_web::main]

 async fn main()->Result<(), std::io::Error>{
    let api=HttpServer::new(||{
        App::new()
        .route("/",web::get().to(index_page))
        .route("/ping",web::get().to(ping))
        
    });
   
    let port=8081;
    
  let api=  api.bind(format!("127.0.0.1:{}",port)).expect("erro  conectar");
    println!("servidor rodando na porta {}",port);
    api.run().await
}


