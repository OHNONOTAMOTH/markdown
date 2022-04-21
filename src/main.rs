use markdown::*;
use rocket::response::content;
use rocket::response::content::{Html, Css, Custom};
use rocket::http::ContentType;
use rocket::*;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use tokio::*;
use toml;

#[derive(Deserialize)]
struct Config {
    pubdir: String,
    cssfile: String,
    favicon: String,
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![getmd, getcssfile, getfavicon])
}

#[get("/<path>")]
fn getmd(path: &str) -> Html<String> {
    let mdp = getconf().unwrap().pubdir + path;
    println!("{}", mdp);
    let mut mdf = File::open(&mdp);
    let mut md = String::new();

    mdf.unwrap().read_to_string(&mut md);

    let html = addcss(gethtm(&md));
    println!("{}", html);
    return content::Html(html);
}

#[get("/css.css")]
fn getcssfile() -> Css<String> {
    let css = File::open(getconf().unwrap().cssfile);
    let mut csstext = String::new();
    css.unwrap().read_to_string(&mut csstext);
    println!("{}", csstext);
    content::Css(csstext)
}

fn getconf() -> Result<Config, toml::de::Error> {
    let mut cFile = File::open("config.toml");
    let mut cString = String::new();

    cFile.unwrap().read_to_string(&mut cString);
    println!("a");
    //let config: Result<Config, toml::de::Error> = toml::from_str(&cString);
    return toml::from_str(&cString);
}

fn gethtm(md: &str) -> String {
    println!("b");
    return markdown::to_html(md);
}

fn addcss(htm: String) -> String {
    println!("{}", htm);
    let mut ohtml = htm.clone();
    let mut h = String::new();
    let hh = h.clone().add(&("
        <head>
            <link rel=\u{0022}stylesheet\u{0022} type=\u{0022}text/css;charset=utf-8\u{0022} href=\u{0022}css.css\u{0022}>
        </head>
        
        <body>".to_owned()
        + 
        &htm
        + 
        &"</body>".to_owned()));
    //h.add(&htm);
    //h.add("</body>");
    ohtml.insert_str(htm.len(), "</body>");
    println!("{}", ohtml);
    return hh;
}

#[get("/favicon.ico")]
fn getfavicon() -> File {
    return File::open(getconf().unwrap().favicon).unwrap();
}