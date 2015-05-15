#![feature(rand)]

extern crate iron;
extern crate router;
extern crate rand;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::__rand::{thread_rng, Rng};
use std::fs::{self, File};
use std::io::Read;

fn main() {
    let mut router = Router::new();

    router.get("/", move |_: &mut Request| {

        let files: Vec<_> = fs::read_dir("./data").unwrap().collect();
        let idx = thread_rng().gen_range(0, files.len());
        let file = files[idx].as_ref().map(|f| f.path()).unwrap();
        let mut data = String::new();
        File::open(file).unwrap().read_to_string(&mut data).unwrap();

        let refresh_seconds = 120;
        let payload = format!(r#"
<!DOCTYPE html>

<head>
<meta charset="utf-8">
<meta http-equiv="refresh" content="{}">
<link rel="stylesheet" type="text/css" href="css.css">
</head>

<body>

<div>The first contribution from...</div>

{}

</body>
<script type="text/javascript" src="js.js"></script>
"#, refresh_seconds, data);
        
        let mime_type: Mime = "text/html".parse().ok().unwrap();

        Ok(Response::with((status::Ok, payload)).set(mime_type))
    });

    router.get("/css.css", move |_: &mut Request| {

        let mut payload = String::new();
        File::open("css.css").unwrap().read_to_string(&mut payload).unwrap();
        
        let mime_type: Mime = "text/css".parse().ok().unwrap();
        
        Ok(Response::with((status::Ok, payload)).set(mime_type))
    });

    router.get("/js.js", move |_: &mut Request| {

        let mut payload = String::new();
        File::open("js.js").unwrap().read_to_string(&mut payload).unwrap();
        
        let mime_type: Mime = "application/javascript".parse().ok().unwrap();
        
        Ok(Response::with((status::Ok, payload)).set(mime_type))
    });

    Iron::new(router).http("localhost:3000").unwrap();
}
