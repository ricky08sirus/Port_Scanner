use std::env::{self, Args};
use std::io::{self,write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender,channel};
use std::thread;

const Max: u16 = 65535;

struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl  Arguments  {
    fn new(args: &[String]) -> Result<Arguments,&'static> {
        if args.len() < 2 {
            return Err("Not many arguments");

        }else if args.len() > 4 {
            return Err("too many arguments");

        }
        let f = args[1].clone();
        

    }

    
}
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (MAX - port) <= num_threads{
            break;

        }
        port += num_threads;
        
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err|{
        if err.contains("help"){
            process::exit(0);
         }else{
            eprintln!("{} problem parsing arguments {}",program,err);

            process::exit(0);

        }
    });

    let num_threads = arguments.threads;
    let (tx,rx) = channel();
    for i in 0..num_threads{
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx,i,arguments.ipaddr, num_threads);

        });

    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p)
    }

    println!("");
    out.sort();

    for v in out {
        println!("{} is open ",v);

    }
    println!("Flag: {}", arguments.get_flag());


}