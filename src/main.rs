use markdown::*;
use rocket::*;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use tokio::*;
use toml;

#[derive(Deserialize)]
struct Config {
    pubdir: String,
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![getmd])
}

#[get("/<path>")]
fn getmd(path: &str) -> String {
    let mdp = getconf().unwrap().pubdir + path;
    println!("{}", mdp);
    let mut mdf = File::open(&mdp);
    let mut md = String::new();

    mdf.unwrap().read_to_string(&mut md);

    format!("{}", &md)
}

fn getconf() -> Result<Config, toml::de::Error> {
    let mut cFile = File::open("config.toml");
    let mut cString = String::new();

    cFile.unwrap().read_to_string(&mut cString);

    //let config: Result<Config, toml::de::Error> = toml::from_str(&cString);
    return toml::from_str(&cString);
}
