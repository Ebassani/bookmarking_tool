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
    let mut search = true;
    let mut redirect = String::new();
    for code in utils::web_codes() {
        if code == command {
            redirect = utils::test(&cmd, &command);
            search = false;
            break;
        }
    }
    if search {
        redirect= utils::search_direct(&cmd)
    }
    
    Redirect::to(redirect)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, command])
}
