//#![deny(warnings)]

// please excuse my poor rust, this is my first time

extern crate toml;
extern crate rustc_serialize;

use std::fs::File;
use std::env;
use std::io;
use std::io::prelude::*;

use toml::Value;
use rustc_serialize::json::Json;

extern crate hyper;
extern crate env_logger;

use hyper::Client;
use hyper::header::Connection;

use std::net::{Ipv4Addr, UdpSocket};

//use rustc_serialize::hex::FromHex;

extern crate sha1;

extern crate msgpack;

fn main() {

    let game_name = "Psilly";
    let game_version = "0.0.1";

    println!("Starting {} Server v.{}...\n", game_name, game_version);

    // config file test

    let mut args = env::args();
    let mut input = String::new();
    let filename = if args.len() > 1 {
        let name = args.nth(1).unwrap();
        File::open(&name).and_then(|mut f| {
            f.read_to_string(&mut input)
        }).unwrap();
        name
    } else {
        /*
        io::stdin().read_to_string(&mut input).unwrap();
        "<stdin>".to_string()
        */
        println!("Location of config file psillyd.toml must be specified as first argument");
        return;
    };

    let mut parser = toml::Parser::new(&input);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         filename, loline, locol, hiline, hicol, err.desc);
            }
            return
        }
    };

    let json = convert(Value::Table(toml));
    println!("{}", json.pretty());




    // http test

    //env_logger::init().unwrap();

    let url = "http://gsl.pow7.com/announce/".to_string();

    let url = url + "?game_name=" + game_name;
    let url = url + "&game_version=" + game_version;

    //let table = Value::Table(parser.parse());
    //let server_name = table.lookup("server.name");
    //println!("Name:\n{}", server_name );

    let game_mode = "Normal";
    let server_port = 42002;
    let server_name = "SERVERNAME";
    let server_password = "PASSWORD";
    let max_players = 512;

    let url = url + "&game_mode=" + game_mode;
    let url = url + "&port=" + &server_port.to_string();
    let url = url + "&name=" + server_name;
    let url = url + "&password=" + server_password;
    let url = url + "&max_players=" + &max_players.to_string();

    println!("Request URL:\n{}", url);

    let client = Client::new();

    let mut announce_result = client.get(&*url)
        .header(Connection::close())
        .send().unwrap();

    println!("Response: {}", announce_result.status);
    //println!("Headers:\n{}", announce_result.headers);

    io::copy(&mut announce_result, &mut io::stdout()).unwrap();


    // udp ping/pong test

    // listen socket
    let udp_socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), server_port)).unwrap();

    // pong socket
    let pong_ip = Ipv4Addr::new(69, 172, 205, 90);
    let pong_port = 42001;


    // pong data
    //let mut pong_data = "pong".to_string();
    let pong_data = "pong1234512345123451234512345123".to_string();

    // test data
    //let server_log_id = 262;
    //let nonce = "24c148a156046268f0259fde5e37640b8041786d".from_hex();
    //let session = "c891a5a5679a10a8fcdb38d959e048aa05c831fb".from_hex();


    let mut m = sha1::Sha1::new();
    m.update("test".as_bytes());
    //m.update(nonce);
    //m.update(session);
    //let sha1_hash = m.digest();

    println!("\nsha1_hash: {}", m.hexdigest());

    //let player_count = 42;


    // server log id (4)
    //pong_data = pong_data + "0262".from_hex();

    // sha1 hash (20)
    //pong_data = pong_data + sha1_hash;

    // player count (2)
    //pong_data = pong_data + &player_count.to_string().from_hex();

    // send pong
    udp_socket.send_to(pong_data.as_bytes(), (pong_ip, pong_port)).unwrap();




    // Send a reply to the socket we received data from
    //let buf = &mut buf[..amt];
    //buf.reverse();
    //try!(udp_socket.send_to(buf, &src));

    //drop(udp_socket); // close the socket



    //let demo_msgpack = msgpack::Encoder::to_msgpack(&as_bytes).ok().unwrap();

    //println!("Encoded: {}", wtf.to_string());

    // processing incoming udp packets

    let mut buf = [0; 48];

    println!("Waiting for UDP packets...");

    loop {

        let result = udp_socket.recv_from(&mut buf);
        println!("Got: {:?}", result);
        //println!("buf.len(): {:?}", buf.len());

        //let pong_data = "pong1234512345123451234512345123".to_string();  // pong + server_log_id(4) + sha1(20) + player_count(2)

        udp_socket.send_to(pong_data.as_bytes(), (pong_ip, pong_port)).unwrap();

    }

    //drop(udp_socket);

}


// used by config file test

fn convert(toml: Value) -> Json {
    match toml {
        Value::String(s) => Json::String(s),
        Value::Integer(i) => Json::I64(i),
        Value::Float(f) => Json::F64(f),
        Value::Boolean(b) => Json::Boolean(b),
        Value::Array(arr) => Json::Array(arr.into_iter().map(convert).collect()),
        Value::Table(table) => Json::Object(table.into_iter().map(|(k, v)| {
            (k, convert(v))
        }).collect()),
        Value::Datetime(dt) => Json::String(dt),
    }
}
