mod tomasulo;

use std::env;
use std::io;
use std::io::Write;
use std::io::stdout;


pub use tomasulo::TomasuloSimulator;

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut simulator = TomasuloSimulator::new();
    simulator.load_nel(&args[1]);
    loop{
        print!("\n");
        stdout().flush();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)
            .expect("Failed to read a line");

        let s : &str = if buf.ends_with("\n") {
            &buf[0..buf.len() - 1] 
        } else {
            &buf
        };

        match s {
            "s" => {
                simulator.step();
            }
            "q" => {
                break;
            }
            "show" => {
                simulator.show();
            }
            _ => {
                println!("fault instruction {} ", s);
            }
        }
    }
}
