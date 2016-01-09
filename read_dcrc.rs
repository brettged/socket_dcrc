#![feature(convert)]

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
  let stream1 = stream.try_clone().unwrap();
  let stream2 = stream.try_clone().unwrap();
  //a two millisecond timeout is fine for 'rd XX' commands
  //but 'rt' needs more time
  //let two_ms = Duration::new(0,5000);
  //note that the clones do not inherit this timeout
  let two_s = Duration::new(2,0);
  stream.set_read_timeout(Some(two_s)).unwrap();

  //loop {
    /*
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
    */
 
    let write_str = "rt\r\n";
    let _ = stream.write(write_str.as_bytes());

    //let response = stream.read(&mut [0; 128]);
    // the first ten bytes tell how many triggers
    // need to be read out
    let mut handle = stream1.take(10);

    let mut rx_header = vec![0; 10];
    handle.read(&mut rx_header).unwrap();
    rx_header.truncate(8);
    let num_triggers = String::from_utf8(rx_header).unwrap();
    let num_triggers_u32 = u32::from_str_radix(num_triggers.as_str(),16).unwrap();
    println!("number of triggers is {}", num_triggers_u32);

    // now that we know the number of triggers,
    // read the rest of the output
    // each trigger is 10 bytes of data
    // the first eight bytes are the characters of the hex number
    // the last two bytes are \r and \n characters
    handle = stream2.take(10*num_triggers_u32 as u64);
    let mut rx = vec![0; 10*num_triggers_u32 as usize];
    handle.read(&mut rx).unwrap();

    let mut addr = rx.split('\r\n').collect();

    //println!("{:?}",rx);
    for dat in &addr {
      println!("{:b} ",dat);
      //print!("{:#X} ",dat);
    }

    println!("the characters are: \n{}",String::from_utf8(rx).unwrap());
  //}
}
