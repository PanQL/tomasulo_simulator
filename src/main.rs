mod tomasulo;

pub use tomasulo::TomasuloSimulator;

use std::env;

extern crate gio;
extern crate gtk;

use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Builder, Button,
};
use std::cell::RefCell;
use std::sync::Arc;

fn main() {
    gtk::init().unwrap();
    let glade_src = include_str!("test_ui.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("app_window").expect("Couldn't get window");
    let step: Button = builder.get_object("step").expect("Couldn't get button1");

    let mut tomasulo = TomasuloSimulator::new(builder);
    let args : Vec<String> = env::args().collect();
    tomasulo.load_nel(&args[1]);

    let simulator = Arc::new(RefCell::new(tomasulo));
    {
        let mut simu = simulator.borrow_mut();
        while simu.still() {
            simu.step();
        }
        simu.show_inst_table();
    }
    step.connect_clicked(move |_| {
       simulator.borrow_mut().step();
    });
    window.show_all();
    gtk::main();
}
