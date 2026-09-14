//! composes: verb-menu, interaction-resolver, response-presenter, path-solver
//!
//! A loopback-only graphical client. The browser draws; Rust owns every game transition.

use play::screen::{Scene, Screen};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !(2..=3).contains(&args.len()) {
        return Err("usage: screen <game> <scene> [port, default 8787]".into());
    }
    let game = play::load(&std::fs::read_to_string(&args[0])?, &args[0])
        .map_err(|errors| errors.join("\n"))?;
    let scene = Scene::parse(&std::fs::read_to_string(&args[1])?, &game)?;
    let mut screen = Screen::new(&game, scene)?;
    let port: u16 = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(8787);
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let host = listener.local_addr()?.to_string();
    println!("Play at http://{host} (Ctrl-C to stop)");
    for connection in listener.incoming() {
        let mut stream = connection?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        if let Err(error) = serve(&mut stream, &mut screen, &host) {
            eprintln!("request: {error}");
        }
    }
    Ok(())
}

fn serve(stream: &mut TcpStream, screen: &mut Screen, host: &str) -> std::io::Result<()> {
    let mut reader = BufReader::new(&mut *stream);
    let mut line = String::new();
    reader.by_ref().take(8193).read_line(&mut line)?;
    let first = line.clone();
    let request: Vec<_> = first.split_whitespace().collect();
    if request.len() != 3 {
        return Ok(());
    }
    let (method, path) = (request[0], request[1]);
    let (mut origin, mut received_host, mut length) = (String::new(), String::new(), None);
    let mut bytes = line.len();
    loop {
        line.clear();
        if reader.by_ref().take(8193).read_line(&mut line)? == 0 {
            return Ok(());
        }
        bytes += line.len();
        if bytes > 16384 {
            return reply(
                stream,
                "431 Request Header Fields Too Large",
                "text/plain",
                "headers too large",
            );
        }
        if line == "\r\n" {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            match key.to_ascii_lowercase().as_str() {
                "origin" => origin = value.trim().into(),
                "host" => received_host = value.trim().into(),
                "content-length" => {
                    if length.is_some() {
                        return reply(stream, "400 Bad Request", "text/plain", "duplicate length");
                    }
                    length = value.trim().parse::<usize>().ok();
                }
                "transfer-encoding" => {
                    return reply(
                        stream,
                        "400 Bad Request",
                        "text/plain",
                        "transfer encoding unsupported",
                    )
                }
                _ => {}
            }
        }
    }
    if received_host != host || (method == "POST" && origin != format!("http://{host}")) {
        return reply(
            stream,
            "403 Forbidden",
            "text/plain",
            "same-origin loopback requests only",
        );
    }
    match (method, path) {
        ("GET", "/") => reply(
            stream,
            "200 OK",
            "text/html; charset=utf-8",
            include_str!("../../assets/index.html"),
        ),
        ("GET", "/client.js") => reply(
            stream,
            "200 OK",
            "text/javascript; charset=utf-8",
            include_str!("../../assets/client.js"),
        ),
        ("POST", "/api") => {
            let Some(length) = length.filter(|n| *n <= 1_048_576) else {
                return reply(
                    stream,
                    "413 Content Too Large",
                    "text/plain",
                    "missing or excessive body length",
                );
            };
            let mut body = vec![0; length];
            reader.read_exact(&mut body)?;
            let body = match std::str::from_utf8(&body) {
                Ok(body) => body,
                Err(_) => return reply(stream, "400 Bad Request", "text/plain", "invalid UTF-8"),
            };
            let (command, value) = body.split_once('\n').unwrap_or((body, ""));
            let (message, saved) = match screen.command(command, value) {
                Ok(saved) => (String::new(), saved),
                Err(error) => (error, None),
            };
            reply(
                stream,
                "200 OK",
                "application/json; charset=utf-8",
                &screen.frame(&message, saved.as_deref()),
            )
        }
        _ => reply(stream, "404 Not Found", "text/plain", "not found"),
    }
}

fn reply(stream: &mut TcpStream, status: &str, kind: &str, body: &str) -> std::io::Result<()> {
    write!(stream,"HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nContent-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; frame-ancestors 'none'\r\n\r\n{body}",body.len())
}
