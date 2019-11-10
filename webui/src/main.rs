use stdweb::*;
use enclose::enclose;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::InputElement;
use stdweb::web::document;
use stdweb::web::event::ClickEvent;

//https://github.com/koute/stdweb/blob/master/examples/todomvc/src/main.rs

fn main() {
    js! { Rust.webui.then(function(webui) { window.WebUI = webui } ); }


    let print_hello = |name: String| {
        println!("Hello, {}!", name);
    };

    console!(log, "Hello, World, console macro!");
    println!("Hello println");
    js! {
        console.log( "Hello "  + "!" );

        var print_hello = @{print_hello};
        print_hello( "Bob" );
        print_hello.drop(); // Necessary to clean up the closure on Rust's side.
    }
    let input: InputElement = document().create_element("input").unwrap().try_into().unwrap();
    let button = document().create_element("button").unwrap();
    input.set_text_content("vlad");
    button.set_text_content("OK");

    button.add_event_listener(enclose!( (input) move |_: ClickEvent| {
        input.set_raw_value("Clicked");
     }));


//    let body: HtmlElement = document().body().unwrap().try_into().unwrap();

    document().body().unwrap().append_child(&input);
    document().body().unwrap().append_child(&button);


//    let docElement = document().document_element().unwrap().try_into().unwrap();
}

#[js_export]
fn print_from_js(param: &str) { js! { console.log("Hello, World, console macro! - " + @{param}); } }