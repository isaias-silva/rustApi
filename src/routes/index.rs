use actix_web::*;

#[path="../datainfo/datainfo.rs"]
mod datainfo;
use datainfo::*;
pub async fn index_page()->Result<HttpResponse>{
    let main_component=include_str!("../../public/components/main.html");

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(include_str!("../../public/index.html")
    .replace("NAME", &get_name())
    .replace("GITHUB", &get_github()).replace("CONTEUDO", main_component)))
    
}