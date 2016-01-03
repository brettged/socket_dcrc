use std::env;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

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

  let mut stream = TcpStream::connect(&addr[..]).unwrap();
  //a two millisecond timeout is fine for 'rd XX' commands
  //but 'rt' needs more time
  //let two_ms = Duration::new(0,5000);
  let two_s = Duration::new(2,0);
  stream.set_read_timeout(Some(two_s)).unwrap();

  //loop {
    println!("Enter a DCRC command: ");
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
    let mut rx = Vec::new();
    let mut keep_reading = true;
    while keep_reading {
       let mut rx_sub = vec![0; 10];
       match stream.read(&mut rx_sub) {
          Ok(bytes) if bytes > 0 => {rx_sub.truncate(bytes); rx.extend(rx_sub.into_iter());},
          Ok(_) => keep_reading = false,
          Err(_) => keep_reading = false,
       } 
    }


    //println!("{:?}",rx);
    print!("{}: ", tx.trim());
    //print!("{} bytes ", bytes);
    for dat in &rx {
      print!("{:#X} ",dat);
    }
    println!("");

    println!("the characters are: {}",String::from_utf8(rx).unwrap());
  //}
}
