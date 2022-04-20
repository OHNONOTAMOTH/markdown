use rocket::*;
use std::fs::File;
use std::io::Read;
use tokio::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![getmd])
}

#[get("/<path>")]
async fn getmd(path: &str) -> String {
    let mdp = "/home/moth/mdblog-be/pubfiles/".to_owned() + path;
    println!("{}", mdp);
    let mut mdf = File::open(&mdp);
    let mut md = String::new();

    mdf.unwrap().read_to_string(&mut md);

    format!("{}", &md)
}
