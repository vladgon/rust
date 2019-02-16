use stdweb::*;

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
}

#[js_export]
fn print_from_js(param: &str) {
    js! { console.log("Hello, World, console macro! - " + @{param}); }
}