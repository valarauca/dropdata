
use dropdata_parser::{pretty_json,parse};

use std::io::{stdin,stdout,Read,BufRead,Write,Stdout};
use std::process::exit;

fn main() {

    let mut s = String::with_capacity(4096);
    match stdin().lock().read_to_string(&mut s) {
        Ok(_) => { },
        Err(e) => {
            eprintln!("error reading stdin: {:?}", e);
            exit(1);
        }
    };
    let data = match parse(&s) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("failed to parse data");
            exit(1);
        }
    };
    let pretty_data = pretty_json(&data);
    stdout().lock().write_all(pretty_data.as_bytes()).unwrap();
    stdout().lock().write_all(b"\n").unwrap();
}
