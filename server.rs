use std::os;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::BufferedReader;
use std::io::File;


use std::str;

//fn main() {
//    for target in os::args().iter() {
//        scan_file(target);
//    }
//}

/*fn scan_file(path_str: &String) {
    let path = Path::new(path_str.as_bytes());
    let file = File::open(&path);
    let mut reader = BufferedReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(s) => {
                match s.as_slice() {
                    "```\n" => print!("<code>\n"),
                    _       => print!("{}", s.as_slice()),
                }
            }
            Err(_) => return,
        }
    }
}*/

fn get_file_string(path_str: &String) -> String{
    let path = Path::new(path_str.as_bytes());
    let file = File::open(&path);
    let mut reader = BufferedReader::new(file);
    reader.read_to_string().unwrap()
}


fn main() {

    let listener = TcpListener::bind("127.0.0.1:8001");
    let mut acceptor = listener.listen();

    let ref file_to_host = os::args()[1];
    let html = get_file_string(file_to_host);

    fn handle_client(mut stream: TcpStream, to_host: String) {
        let header: String = "HTTP/1.1 200 OK\nContent-Type: text/html\n\r\n".to_string();
     //   match stream.read_to_string() {
     //       Ok(x) => println!("{}", x),
     //       Err(err) => println!("FAILURE"),
     //   }

        let mut stream_reader = BufferedReader::new(stream.clone());
        println!("{}", stream_reader.read_until(32));
        println!("{}", stream_reader.read_until(46));
 //       println!("{}", String::from_utf8(stream.read_exact(12).unwrap()));
        
        stream.write_str((header + to_host).as_slice());
        // drop(stream);
    }

    for stream in acceptor.incoming() {
        let html = html.clone();
        match stream {
            Err(e) => { println!("{}", e) }
            Ok(stream) => spawn(proc() {
                handle_client(stream,html)
            })
        }
    }

}


