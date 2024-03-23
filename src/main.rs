#![allow(dead_code)]
use std::io;
use server::Server;
use http::request::Request;
use http::method::Method;
use web_handler::WebHandler;
use std::env;

mod server;
mod http;
mod web_handler;
fn main() {
    // println!("Please enter your weight in kg!!!");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let weight:f32 = input.trim().parse().unwrap();
    // let mut mars_weight = calculate(weight);
    // mars_weight = mars_weight * 1000.00;
    // println!("Weight on mars is {}kg", mars_weight);
    // print!("User input is {}", input);
    let default_path = format!("{}/public" ,env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let  server= Server::new("127.0.0.1:8086".to_string());
    server.run(WebHandler::new(public_path));
    
}












