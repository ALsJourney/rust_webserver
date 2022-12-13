use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);


    for stream in listener.incoming() {
        // Get result or panic if error
        let stream = stream.unwrap();


        thread::spawn(|| {
            handle_connection(stream);
        });
        println!("Connection established!");
    }

    // read data from the stream
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = 
    
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();


    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// response must be in the following format
// Status-Line
// headers
// blank line
// message body

// Status-Line = HTTP-Version SP Status-Code SP Reason-Phrase CRLF

// HTTP-Version = "HTTP" "/" 1*DIGIT "." 1*DIGIT

// Status-Code = 3DIGIT

// Reason-Phrase = *<TEXT, excluding CR, LF>

// TEXT = <any OCTET except CTLs, but including LWS>

// CTL = <any US-ASCII control character (octets 0 - 31) and DEL (127)>
// LWS = [CRLF] 1*( SP | HT )
