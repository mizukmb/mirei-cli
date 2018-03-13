extern crate clap;
use clap::{Arg, App};

pub struct Ragulation {
    content: &'static str,
    episode: u16,
    number: u16,
    violator: &'static str,
}

fn main() {
    let mirei = Ragulation{
        content: "廊下を走ってはならない",
        episode: 1,
        number: 3243,
        violator: "真中らぁら",
    };

    let version = "1.0.0";
    let matches = App::new("mirei")
                    .version(version)
                    .author("mizukmb <mizukmb6@gmail.com>")
                    .about("Echo Paprika private academy regulations")
                    .arg(Arg::with_name("episode")
                        .short("e")
                        .long("episode")
                        .help("Echo with episode"))
                    .arg(Arg::with_name("number")
                        .short("n")
                        .long("number")
                        .help("Echo with number"))
                    .arg(Arg::with_name("violator")
                        .short("v")
                        .long("violator")
                        .help("Echo with violator"))
                    .get_matches();

    if matches.is_present("number") {
        println!("{}", mirei.number);
    }
    if matches.is_present("violator") {
        println!("{}", mirei.violator);
    }
    if matches.is_present("episode") {
        println!("{}", mirei.episode);
    }

    println!("{}", mirei.content);
}
