use actix_web::*;
#[path="../datainfo/datainfo.rs"]
mod datainfo;
use datainfo::*;
pub async fn info()->HttpResponse{
    
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(format!("<b>creator</b>:{}<br><b>github</b>:{}",get_name(),get_github()))
}