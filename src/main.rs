#[macro_use] extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/command?<cmd>")]
fn command(cmd: String) -> Redirect {
    let command = utils::get_command(&cmd);
    let redirect = match command {
        "yt" => utils:: test(&cmd),
        "rd" => utils::reddit_redirect(&cmd),
        _ => utils::search_direct(&cmd)
        };
    Redirect::to(redirect)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, command])
}
