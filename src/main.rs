use std::env;

mod cmd;
use cmd::cmd_parse;

mod build;
use build::build_rpm;

fn main() {
    let args : Vec<String> = env::args().collect();
    cmd_parse(args[1..].to_vec());
    build_rpm();
}
