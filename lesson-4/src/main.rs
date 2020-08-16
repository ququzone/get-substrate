use clap::{Arg, App};

fn main() {
  let matches = App::new("String utils")
    .version("v1.0.0")
    .author("ququzone <xueping.yang@gmail.com>")
    .about("string utlis")
    .arg(
      Arg::new("config")
        .short('c')
        .long("config")
        .about("Sets config file")
        .required(true)
        .takes_value(true),
    )
    .arg(
      Arg::new("operate")
        .about("Sets operate")
        .index(1),
    )
    .subcommand(
      App::new("test")
        .about("does test subcommand")
        .arg(Arg::new("hello").short('h').about("subcommand hello values")),
    )
    .get_matches();

  let config = matches.value_of("config").unwrap();
  println!("config file is {}", config);
}