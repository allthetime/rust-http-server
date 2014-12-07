use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::BufferedReader;
use std::io::File;


fn get_file_buffer(path_str: &String) -> Vec<u8> {
    let path = Path::new(path_str.as_bytes());
    let file = File::open(&path);
    let mut reader = BufferedReader::new(file);
    match reader.read_to_end() {
        Ok(x) => x,
        Err(_) => vec![0],
    }
}

// why doesn't this work?

//fn unpack_stream_buffer<T>(stream_reader: &BufferedReader<T>) -> String {
//    match stream_reader.read_until(32) {
//         Ok(buffer) => match String::from_utf8(buffer) {
//             Ok(string) => string,
//             Err(err) => err.to_string(),
//         },
//         Err(err) => err.to_string(),
//     }
//}

fn handle_client(mut stream: TcpStream) {

    let header: String = "HTTP/1.1 200 OK\nContent-Type: text/html\n\r\n".to_string();
    let mut stream_reader = BufferedReader::new(stream.clone());
  
    // read until first space of request to determine HTTP method

    let method = match stream_reader.read_until(32) {
        Ok(buffer) => match String::from_utf8(buffer) {
            Ok(string) => string,
            Err(err) => err.to_string(),
        },
        Err(err) => err.to_string(),
    };

    // read until second space of request to get URI

    let request_uri = match stream_reader.read_until(32) {
        Ok(buffer) => match String::from_utf8(buffer) {
            Ok(string) => string,
            Err(err) => err.to_string(),
        },
        Err(err) => err.to_string(),
    };
   
    println!("request uri: {}", request_uri);
    
    // Redirect root to a.html, otherwise parse file name

    let get_file = match request_uri.as_slice() {
        "/ " => "a.html".to_string(),
        _ => request_uri.as_slice().slice(1,request_uri.as_slice().len()-1).to_string(),
    };

    // If file doesn't exist 404, otherwise serve file

    match get_file_buffer(&get_file).as_slice() {
        [0] => stream.write_str( ( "404: file not found".to_string()).as_slice() ).unwrap(),
        _ => stream.write( (header.into_bytes() + get_file_buffer(&get_file)).as_slice() ).unwrap()
    }
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8666");
    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { println!("{}", e) }
            Ok(stream) => spawn(proc() {
                handle_client(stream)
            })
        }
    }
}


