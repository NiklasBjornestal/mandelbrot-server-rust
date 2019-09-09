#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::config::{Config, Environment};
use std::io;
use std::env;

#[get("/<min_c_re>/<min_c_im>/<max_c_re>/<max_c_im>/<xsize>/<ysize>/<inf_n>")]
fn mandelbrot(min_c_re :f32, min_c_im :f32, max_c_re :f32, max_c_im :f32, xsize :u32 ,ysize :u32, inf_n :u32) -> Vec<u8> {
    let mut image: Vec<u8> = Vec::new();
	for y in 0..ysize {
		for x in 0..xsize {
			let mut zx = 0.0;
			let mut zy = 0.0;
			let cX = x as f32 * (max_c_re - min_c_re) / xsize as f32 + min_c_re;
			let cY = y as f32 * (max_c_im - min_c_im) / ysize as f32 + min_c_im;
            let mut iter = inf_n;
			while (zx * zx + zy * zy < 4.0 && iter > 0) {
				let tmp = zx * zx - zy * zy + cX;
				zy = 2.0 * zx * zy + cY;
				zx = tmp;
				iter -= 1;
			}
			image.push((iter % 256) as u8);
		}
	}
	image
}

fn main() -> Result<(), rocket::config::ConfigError> {
    let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		eprintln!("Usage:\n{} server port", args[0]);
	}
	let server = &args[1];
	let port = args[2].parse().unwrap();
	
    let config = Config::build(Environment::Staging)
        .address(server)
        .port(port)
        .finalize()?;

    rocket::custom(config)
        .mount("/mandelbrot", routes![mandelbrot])
        .launch();
//	rocket::ignite().mount("/hello", routes![hello]).launch();
	Ok(())
}