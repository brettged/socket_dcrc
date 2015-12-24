use std::env;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
  let args: Vec<String> = env::args().collect();
  let ref addr = args[1];
  println!("{}",&addr[..]);

  let mut stream = TcpStream::connect(&addr[..]).unwrap();

  loop {
    println!("Enter the DCRC register to read: ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("failed to read line");

    let guess: u32 = guess.trim().parse()
        .ok()
        .expect("please type a DCRC register:");


    let write_str = format!("rd {}\r\n",guess);

    let _ = stream.write(write_str.as_bytes());

    //let response = stream.read(&mut [0; 128]);
    let mut rx = vec![0; 10];
    stream.read(&mut rx).unwrap();

    //println!("{:?}",rx);
    print!("register 0x{}: ", guess);
    for dat in &rx {
      print!("{:b} ",dat);
    }
    println!("");
  }
}
