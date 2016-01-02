use std::env;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;


fn check_tx(tx: &String) -> Result<&'static str, &'static str>{
    let check_tx: Vec<&str> = tx.split_whitespace().collect();

    match check_tx[0] {
      "rd" => return Ok("read a register"),
      "rt" => return Ok("read trigger list"),
      "wr" => return Ok("retrieve waveform data"),
      "help" => return Ok("show help menu"),
      _ => return Err("That is not a valid DCRC command."),
    }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let ref addr = args[1];
  println!("{}",&addr[..]);

  let mut stream = TcpStream::connect(&addr[..]).unwrap();

  loop {
    println!("Enter the DCRC register to read: ");
    let mut tx = String::new();

    io::stdin().read_line(&mut tx)
        .ok()
        .expect("failed to read line");
    

    match check_tx(&tx) {
      Ok(_) => {},
      Err(e) => panic!(e.to_string()),
    }

    let write_str = format!("{}\r\n",tx.trim());

    let _ = stream.write(write_str.as_bytes());

    //let response = stream.read(&mut [0; 128]);
    let mut rx = vec![0; 10];
    stream.read(&mut rx).unwrap();

    //println!("{:?}",rx);
    print!("{}: ", tx.trim());
    for dat in &rx {
      print!("{:b} ",dat);
    }
    println!("");
  }
}
