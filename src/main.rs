use std::io::{BufReader, BufWriter, Cursor};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use ciborium::{de, ser};
use ciborium::value::Value;

fn handle_connection(mut stream: TcpStream) {
    let mut data = Vec::new();
    let mut buf_stream = BufReader::new(&stream);

    println!("Connection established");

    // Array with a fixed size
    let mut rx_bytes = [0u8; 2048];
    loop {
        // Read from the current data in the TcpStream
        let bytes_read = buf_stream.read(&mut rx_bytes).unwrap();

        // However many bytes we read, extend the `received` string bytes
        data.extend_from_slice(&rx_bytes[..bytes_read]);

        // If we didn't fill the array
        // stop reading because there's no more data (we hope!)
        if bytes_read < 2048 {
            break;
        }
    }

    let input: Value = de::from_reader(&*data).unwrap();

    let mut buff = Cursor::new(Vec::new());

    ser::into_writer(&input, &mut buff).unwrap();

    println!("{:?}", &buff.get_ref());

    stream.write(&buff.get_ref());
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9493").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
