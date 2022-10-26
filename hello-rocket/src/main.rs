#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> &'static str {
    "Dette er en test :) <a href='/'>Tilbage</>"
}

#[get("/anden")]
fn anden() -> &'static str {
    "NR 2 <a href='/'>Tilbage</>"
}

#[get("/første")]
fn første() -> &'static str {
    "NR 1 <a href='/'>Tilbage</>"
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, test])
        .mount("/v/", routes![første, anden])
}