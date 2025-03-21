use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

struct Server {
    address: String,
}

impl Server {
    /// Fungsi build sebagai pengganti new
    fn build(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    /// Menjalankan server
    fn run(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Server berjalan di {}", self.address);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            thread::spawn(|| {
                handle_connection(stream);
            });
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    // Menggunakan `build` untuk membuat server
    let server = Server::build("127.0.0.1:7878");
    server.run();
}
