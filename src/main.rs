use std::{
    io::{self, stdout, Write},
    process::exit,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    timeout: i32,

    #[arg(short, long, default_value_t = String::from("Exiting with code 0 in"))]
    output: String,
}

fn main() {
    let args = Args::parse();

    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let tx_usr_imp = tx.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(args.timeout.try_into().unwrap()));
        tx.send(false).unwrap();
    });

    thread::spawn(move || {
        let mut count_down = args.timeout;
        loop {
            thread::sleep(Duration::from_secs(1));
            print!("\r{}      ", count_down);
            stdout().flush().unwrap();
            count_down -= 1;
        }
    });

    thread::spawn(move || {
        println!(
            "Press enter to continue (reverting in {} seconds)",
            args.timeout
        );
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();
        tx_usr_imp.send(true).unwrap();
    });

    match rx.recv() {
        Ok(exit_good) => match exit_good {
            true => {
                exit(0);
            }
            false => {
                exit(1);
            }
        },
        Err(_) => exit(1),
    }
}
