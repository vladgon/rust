use actix_web::guard::GuardContext;
use actix_web::http::header::ContentType;

pub fn accept_json(ctx: &GuardContext) -> bool {
    match ctx.header::<ContentType>() {
        Some(content_type) => ContentType::json() == content_type,
        None => false,
    }
}
