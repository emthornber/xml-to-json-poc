use log::{error, info};
use simple_logger::SimpleLogger;

use std::{env, string::String};
use time::macros::format_description;

use xml_to_json::CBusInterface;
use xml_to_json::MergModuleDataSet;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .env()
        .with_timestamp_format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .init()
        .unwrap();
    let args: Vec<String> = env::args().collect();
    info!("Input file '{}'", args[1]);
    let fcu_file = args[1].as_str();
    let fc = MergModuleDataSet::populate(&fcu_file);
    match fc {
        Ok(fcu_config) => {
            let cbus_events = CBusInterface::new(fcu_config.clone());
            println!("{}", cbus_events.pretty_print());
        }
        Err(e) => {
            error!("{} '{}'", e, fcu_file);
        }
    }
}
