// Adapted from https://github.com/ninjasource/embedded-websocket/blob/master/examples/server.rs

// The MIT License (MIT)
// Copyright (c) 2019 David Haig

// Demo websocket server that listens on localhost port 1337.
// If accessed from a browser it will return a web page that will automatically attempt to
// open a websocket connection to itself. Alternatively, the client.rs example can be used to
// open a websocket connection directly. The server will echo all Text and Ping messages back to
// the client as well as responding to any opening and closing handshakes.
// Note that we are using the standard library in the demo but the websocket library remains no_std

use embedded_websocket::{
    framer::{Framer, FramerError},
    WebSocketContext, WebSocketSendMessageType, WebSocketServer,
};
use jiff::Timestamp;
use std::str::Utf8Error;
use std::{
    io::{Read, Write},
    usize,
};
use std::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

type Result<T> = std::result::Result<T, WebServerError>;

#[allow(unused)]
#[derive(Debug)]
pub enum WebServerError {
    Io(std::io::Error),
    Framer(FramerError<std::io::Error>),
    WebSocket(embedded_websocket::Error),
    Utf8Error,
}

impl From<std::io::Error> for WebServerError {
    fn from(err: std::io::Error) -> WebServerError {
        WebServerError::Io(err)
    }
}

impl From<FramerError<std::io::Error>> for WebServerError {
    fn from(err: FramerError<std::io::Error>) -> WebServerError {
        WebServerError::Framer(err)
    }
}

impl From<embedded_websocket::Error> for WebServerError {
    fn from(err: embedded_websocket::Error) -> WebServerError {
        WebServerError::WebSocket(err)
    }
}

impl From<Utf8Error> for WebServerError {
    fn from(_: Utf8Error) -> WebServerError {
        WebServerError::Utf8Error
    }
}

pub(crate) fn send_temp_to_client(
    addr: &str,
    temp_receiver: mpsc::Receiver<u16>,
) {
    let mut read_buf = [0; 4000];
    let mut read_cursor = 0;

    let mut write_buf = [0; 4000];
    let mut websocket = WebSocketServer::new_server();

    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening on: {}", addr);
    println!("Getting next incoming conn");
    let mut stream = listener.incoming().next().unwrap().unwrap();
    println!("Client connected {}", stream.peer_addr().unwrap());

    let websocket_context =
        read_header(&mut stream, &mut read_buf, &mut read_cursor)
            .unwrap()
            .unwrap();

    let mut framer = Framer::new(
        &mut read_buf,
        &mut read_cursor,
        &mut write_buf,
        &mut websocket,
    );

    // complete the opening handshake with the client
    framer.accept(&mut stream, &websocket_context).unwrap();
    println!("Websocket connection opened");

    loop {
        let temp = temp_receiver.recv().unwrap();
        let timestamp = Timestamp::now().as_second();

        let mut data = timestamp.to_be_bytes().to_vec();
        data.append(&mut temp.to_be_bytes().to_vec());

        framer
            .write(&mut stream, WebSocketSendMessageType::Binary, true, &data)
            .unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn read_header(
    stream: &mut TcpStream,
    read_buf: &mut [u8],
    read_cursor: &mut usize,
) -> Result<Option<WebSocketContext>> {
    loop {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut request = httparse::Request::new(&mut headers);

        let received_size = stream.read(&mut read_buf[*read_cursor..])?;

        match request
            .parse(&read_buf[..*read_cursor + received_size])
            .unwrap()
        {
            httparse::Status::Complete(len) => {
                // if we read exactly the right amount of bytes for the HTTP header then read_cursor would be 0
                *read_cursor += received_size - len;
                let headers = request.headers.iter().map(|f| (f.name, f.value));
                match embedded_websocket::read_http_header(headers)? {
                    Some(websocket_context) => match request.path {
                        Some("/") => {
                            return Ok(Some(websocket_context));
                        }
                        _ => return_404_not_found(stream, request.path)?,
                    },
                    None => {
                        handle_non_websocket_http_request(
                            stream,
                            request.path,
                        )?;
                    }
                }
                return Ok(None);
            }
            // keep reading while the HTTP header is incomplete
            httparse::Status::Partial => *read_cursor += received_size,
        }
    }
}

fn handle_non_websocket_http_request(
    stream: &mut TcpStream,
    path: Option<&str>,
) -> Result<()> {
    println!("Received file request: {:?}", path);

    return_404_not_found(stream, path)?;

    Ok(())
}

fn return_404_not_found(
    stream: &mut TcpStream,
    unknown_path: Option<&str>,
) -> Result<()> {
    println!("Unknown path: {:?}", unknown_path);
    let html = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    stream.write_all(&html.as_bytes())?;
    Ok(())
}
