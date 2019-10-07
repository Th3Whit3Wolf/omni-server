extern crate actix_web;

extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufReader},
};

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::{guard, middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use fake_db::*;
//use ifaces::Kind::Ipv6;
use conf::*;
use rustls::{
    internal::pemfile::{certs, rsa_private_keys},
    NoClientAuth, ServerConfig,
};
use termion::color;

mod conf;
mod fake_db;

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    title: &'static str,
    page_name: &'static str,
}

#[derive(Template)]
#[template(path = "404.html")]
struct PageNotFound {
    title: &'static str,
    page_name: &'static str,
}

#[derive(Template)]
#[template(path = "downloads.html")]
struct Downloads {
    title: &'static str,
    page_name: &'static str,
}

#[derive(Template)]
#[template(path = "packages.html")]
struct PackagesQuery<'a> {
    title: &'static str,
    page_name: &'static str,
    packages: Vec<Package>,
    package_num: usize,
    total_pages: i32,
    page: i32,
    kernel: &'a str,
    arch: &'a str,
    repo: &'a str,
    release: &'a str,
}

fn pg404() -> Result<HttpResponse> {
    let pg = PageNotFound {
        title: "Oh no...",
        page_name: "404",
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(pg))
}

fn index() -> Result<HttpResponse> {
    let index = Index {
        title: "Welcome to Neo OS!",
        page_name: "Index",
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(index))
}

fn packages(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    const ITEM_PER_PAGE: u8 = 10;
    const PAGE_NUMBER: u8 = 0; //Starts at 0
    let s = String::new();
    let kernel = query.get("kernel").unwrap_or(&s);
    let arch = query.get("arch").unwrap_or(&s);
    let repo = query.get("repo").unwrap_or(&s);
    let release = query.get("release").unwrap_or(&s);
    let page = match query.get("page") {
        Some(page) => match page.parse::<i32>() {
            Ok(page) => page,
            Err(_error) => 1 as i32,
        },
        _ => 1 as i32,
    };
    let (table_item, total_pages, package_num) = get_page_items(PAGE_NUMBER, ITEM_PER_PAGE);
    let pkg = {
        PackagesQuery {
            title: "Packages for Neo OS",
            page_name: "Packages",
            packages: table_item,
            package_num,
            page,
            total_pages,
            kernel,
            arch,
            repo,
            release,
        }
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(pkg))
}

fn downloads() -> Result<HttpResponse> {
    let dl = Downloads {
        title: "Download Neo OS",
        page_name: "Downloads",
    }
    .render()
    .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(dl))
}

fn main() -> io::Result<()> {
    let settings = Settings::new().expect("Error parsing config/Settings.toml");
    let debug = settings.server.debug;
    let address = settings.server.host;
    let port = settings.server.port;
    let cert_path = settings.cert.cert;
    let key_path = settings.cert.key;
    let sled_db_path = settings.database.path;

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    /* Default Actix Server Settings
     * workers = Number of Available Logical CPUs as thread counts
     * backlog = 2048
     * maxconn(Max Connections per Worker Thread) = 25k
     * maxxconnrate = 256
     * keep_alive = 5 sec
     * client_timeout = 5000 ms (5 sec)
     * client_shutdown = 5000 ms (5 sec)
     * shutdown_timeout = 30 sec
     */
    if debug {
        println!("{}🔧 Configured for debug", color::Fg(color::Blue));
        println!(
            "{}    => {}SSL Cert Path: {}{}",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
            cert_path
        );
        println!(
            "{}    => {}SSL Key Path: {}{}",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
            key_path
        );
        println!(
            "{}    => {}Database Path: {}{}",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
            sled_db_path
        );
        println!(
        "{}    => {}Max Concurrent Connections per Worker: {}25k",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
        );
        /*
        println!(
            "{}    => {}Max Concurrent Connections per Worker Rate: {}256",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
        println!(
            "{}    => {}Client Timeout: {}5s",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
        println!(
            "{}    => {}Client Shutdown: {}5s",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
        println!(
            "{}    => {}Shutdown Timeout: {}30s",
            color::Fg(color::White),
            color::Fg(color::Blue),
            color::Fg(color::White),
        );
    */
    } else {
        println!("{}🔧 Configured for production", color::Fg(color::Blue));
    }
    println!(
        "{}    => {}Address: {}{}",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
        address
    );
    println!(
        "{}    => {}Port: {}{}",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
        port
    );
    println!(
        "{}    => {}Log: {}Normal",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
    );
    println!(
        "{}    => {}Workers: {}12",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
    );
    println!(
        "{}    => {}Rustls: {}Enabled",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
    );
    println!(
        "{}    => {}Keep Alive: {}5s",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
    );
    println!(
        "{}    => {}Serving On: {}https://{}:{}/",
        color::Fg(color::White),
        color::Fg(color::Blue),
        color::Fg(color::White),
        address, port
    );
    
    println!("{}", color::Fg(color::Reset));

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
    let key_file = &mut BufReader::new(File::open(key_path).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    let sys = actix_rt::System::new("Omni Server");

    HttpServer::new(|| {
        App::new()
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(Logger::new("%r"))
            // register simple route, handle all methods
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/downloads").route(web::get().to(downloads)))
            .service(web::resource("/packages").route(web::get().to(packages)))
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(pg404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind_rustls(address + ":" + &port.to_string(), config)?
    .start();
    sys.run()
}

fn get_page_items(page_number: u8, item_per_page: u8) -> (Vec<Package>, i32, usize) {
    let mut table_item: Vec<Package> = Vec::with_capacity(item_per_page as usize);
    let db = read_db(String::from("sled_db.db"));
    let len = &db.len();
    for i in (page_number * item_per_page)..=(page_number * item_per_page + (item_per_page - 1)) {
        let string = String::from("systemd") + &i.to_string();
        if let Some(bytes) = db.get(string).unwrap() {
            let pkg: Package =
                bincode::deserialize(&bytes).expect("Error: converting bincode to package");
            table_item.push(pkg);
        }
    }
    (table_item, (db.len() / item_per_page as usize) as i32, *len)
}

/*
    /*
    * GET = Green
    * Path = Blue
    * Response sucess = Green
    * Matched = Blue
    * Path = Blue
    * [] = Blue
    * Warning = Yellow
    * Failure = Red
    * 404 Not Found = Red
    * Warning Response = dark yellow
    * text/html = dark yellow
    * Else _ => White
    */

GET / text/html:
    => Matched: GET / [10]
    => Outcome: Success
    => Response succeeded.
GET /packages text/html:
    => Matched: GET /<path..> [10]
    => Warning: Response was `None`.
    => Outcome: Failure
    => Warning: Responding with 404 Not Found catcher.
    => Response succeeded.
*/
