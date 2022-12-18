use actix_web::*;
#[path ="../structs/structs.rs"]

mod structs;
use structs::*;
#[path="../datainfo/datainfo.rs"]
mod datainfo;
use datainfo::*;

pub async fn ping()->Result<HttpResponse>{
    let post=Post{
        title:String::from("PING"),
        text:String::from("PONG")
    };

    let component=include_str!("../../public/components/default.html")
    .replace("TITLE", &post.title)
    .replace("TEXT", &post.text);
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(include_str!("../../public/index.html")
    .replace("NAME", &get_name())
    .replace("GITHUB", &get_github()).replace("CONTEUDO", &component)))
    
}