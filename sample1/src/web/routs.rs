use rocket::response::content;
use rocket::Route;

#[get("/vlad")]
fn vlad() -> &'static str { "Hello, world! Vlad" }

#[get("/")]
fn index() -> &'static str { "Hello, world!" }

#[get("/json/<id>/<name>")]
pub fn get_json(id: String, name: String) -> content::Json<String> {
    let res: serde_json::Value = json!( {
        "hi":id,
        "name":name
    } );
    content::Json(res.to_string())
}

pub fn get() -> Vec<Route> { routes![ index, vlad, get_json] }

