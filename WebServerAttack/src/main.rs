use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;//biblioteca para la salida estandar
use std::fs;
use std::time::Duration;
use std::thread;
use WebServerAttack::ThreadPool;

fn main() {
    //Listener que recibe todas las peticiones//
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);//Cantidad de hilos con los cuales se trabajara//
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
        
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    //Metodoso que tendra el servidor//
    let get = b"GET / HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let put = b"PUT / HTTP/1.1\r\n";
    let head = b"HEAD / HTTP/1.1\r\n";
    let delete = b"DELETE /HTTP/1.1\r\n";
    


    let (status_line,filename) = 
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK","index.html")
        } 
        else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        } 
        else if buffer.starts_with(post) {
            ("HTTP/1.1 200 OK","index.html")
        }
        else if buffer.starts_with(put) {
            ("HTTP/1.1 200 OK","index.html")
        }
        else if buffer.starts_with(head) {
            ("HTTP/1.1 200 OK","index.html")
        }
        else if buffer.starts_with(delete) {
            ("HTTP/1.1 200 OK","index.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND","404.html")
        };
    //lee el archivo//
    let contents = fs::read_to_string(filename).unwrap();

    //respuesta que retorna el servidor//
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    //Muesta la respuesta que se tuvo del servidor//
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

