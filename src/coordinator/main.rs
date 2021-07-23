#[path = "../utils/logger.rs"]
mod logger;

mod coordinator;
mod node_accepted;

use coordinator::Coordinator;
use logger::Logger;

use std::env;
use std::process;
use std::thread;
use std::sync::Arc;

const LOG_FILENAME: &str = "log_coordinator.txt";
const MESSAGE_LOGGER_ERROR: &str = "Unable to open logger file ";

fn usage() -> i32 {
    println!("Usage: cargo r --bin coordinator");
    return -1;
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        process::exit(usage());
    }
    let logger = match Logger::new(LOG_FILENAME) {
        Ok(logger) => Arc::new(logger),
        Err(e) => {
            println!("{} {:?}: {:?}", MESSAGE_LOGGER_ERROR, LOG_FILENAME, e);
            return Err(());
        }
    };

    let coordinator = thread::spawn(move || {
        let coordinator = Coordinator::new(logger);
        coordinator.run();
    });

    match coordinator.join() {
        Ok(()) => println!("Join Coordinator"),
        Err(e) => println!("{:?}", e),
    };

    Ok(())
}
