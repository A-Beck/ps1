// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3


#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};
use std::io::buffered::BufferedReader;
use std::io::File;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut count: int = 0;

fn main() {
    
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    
    for stream in acceptor.incoming() {

        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;

            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            

            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);

            let mut line1 = "";

            for str in request_str.split('\n'){ // get the first line
                line1 = str.clone();
                break;
            }

            
            let mut path = "";

            let temp: ~[&str] = line1.split(' ').collect();
            path = temp[1].clone(); // this gets the path

            let file_content = readFile(path);
            println!("this is from file_content: {:s}",file_content);

            unsafe {    // so we can increment count 'illegally'

            count += 1;

            let response: ~str = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body \\{ background-color: \\#111; color: \\#FFEEAA \\}
                        h1 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red\\}
                        h2 \\{ font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green\\}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty! Total hits = {:d} </h1>
                 <p> reading from file ... <br/>
                 {:s}
                </p>
                 </body></html>\r\n", count, file_content);

            stream.write(response.as_bytes());
            println!("Connection terminates");
            }
        }
    }
}   

fn readFile(x: &str) -> ~str {

    println(x);   // for error checking

    let path = Path::new(x.slice_from(1));  // gets rid of leading '/'
    

    match (path.exists(), path.is_file()){
        
        (true,true) => {  
            println("this is a file!");
            match File::open(&Path::new(path)) {
                Some(file) => {
                    let mut reader = BufferedReader::new(file);
                    let input = reader.read_to_str();
                    println!("{:s}", input);
                    return input;

                }

                None =>{~"Opening file failed!"}
            }

        }
        (false,_) => {~"The item requested is not a file"}
        (_,false) => {~"The item requested is not a file"}
    }

}