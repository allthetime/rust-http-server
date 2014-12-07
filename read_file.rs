use std::os;
use std::io::BufferedReader;
use std::io::File;

fn main() {
    for target in os::args().iter() {
        scan_file(target);
    }
}

fn scan_file(path_str: &String) {
  let path = Path::new(path_str.as_bytes());
  let file = File::open(&path);
  let mut reader = BufferedReader::new(file);
    
  println!("{}",reader.read_to_end());

  
//  for line in reader.lines() {
//      match line {
//          Ok(s) => {
//              match s.as_slice() {
//                  "```\n" => print!("<code>\n"),
//                  _       => print!("{}", s.as_slice()),
//              }
//          }
//          Err(_) => return,
//      }
//  }
}
