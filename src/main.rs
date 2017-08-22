
extern crate clap;
use clap::{Arg, App};
extern crate changelogger;

fn main() {
    let matches = App::new("changelogger")
        .version("0.1")
        .author("Cajetan Puchalski <cajetan.puchalski@gmail.com>")
        .about("A simple tool for updating changelogs")
        .arg(Arg::with_name("add_exercise")
             .short("ae")
             .long("add_exercise")
             .value_name("LANGUAGE")
             .value_name("TITLE")
             .value_name("SOURCE")
             .help("Information about the exercise")
        .takes_value(true))
        .get_matches();

    if let Some(exercise) = matches.values_of("add_exercise") {
        let data: Vec<&str> = exercise.collect();
        changelogger::add_exercise(data[0], data[1], data[2]);
    }

}
