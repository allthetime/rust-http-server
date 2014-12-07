use std::os;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::BufferedReader;
use std::io::File;


fn get_file_string(path_str: &String) -> String{
    let path = Path::new(path_str.as_bytes());
    let file = File::open(&path);
    let mut reader = BufferedReader::new(file);
    match reader.read_to_string() {
        Ok(x) => x,
        Err(_) => "err".to_string(),
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

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8001");
    let mut acceptor = listener.listen();

 //   let ref file_to_host = os::args()[1];
 //   let html = get_file_string(file_to_host);

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

        match get_file_string(&get_file).as_slice() {
            "err" => stream.write_str((header + "404".to_string()).as_slice()).unwrap(),
            _ => stream.write_str((header + get_file_string(&get_file)).as_slice()).unwrap(),
        }
    }

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { println!("{}", e) }
            Ok(stream) => spawn(proc() {
                handle_client(stream)
            })
        }
    }

}


