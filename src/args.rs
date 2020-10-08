use super::constants;
use clap::{App, Arg};

fn make_app() -> clap::App<'static> {
    return App::new(constants::APPNAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about(constants::ABOUT)
        .arg("-c, --command=[CMD] 'Runs a command'")
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(false)
                .index(1),
        );
}


pub fn parse_args() -> clap::ArgMatches {
    return make_app().get_matches();
}