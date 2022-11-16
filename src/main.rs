
#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str{
    "Crypto won't die!"
}

#[launch]
fn rocket()->_{
    rocket::build()
        .mount("/hello", routes![world])
}
