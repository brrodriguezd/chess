use game::chess::Board;

#[allow(dead_code)]
#[macro_use]
extern crate rocket;

mod game;

#[get("/")]
fn index() -> String {
    let tablero = Board::new();
    tablero.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}