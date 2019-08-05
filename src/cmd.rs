use std::collections::HashMap;

fn parse_install_args(args : Vec<String>) -> (usize, Vec<String>, Vec<String>) {
  let mut num_args = 1;
  let mut install_pkgs : Vec<String> = Vec::new();
  let mut install_flags : Vec<String> = Vec::new();

  let a = args[0].clone();
  let rest_args = args[1..].to_vec();

  if !rest_args.is_empty() {
    let (_num_args, _install_pkgs, _install_flags) = parse_install_args(rest_args);
    num_args = _num_args;
    install_pkgs = _install_pkgs;
    install_flags = _install_flags;
  }  

  if a.len() > 2 && &(a[..2]) == "--" {
    install_flags.push(a);
  }
  else if a.len() > 1 && &(a[..1]) == "-" {
    install_flags.push(a);
  } else {
    install_pkgs.push(a);
  }
  return (num_args + 1, install_pkgs, install_flags);
}

pub fn cmd_parse(args : Vec<String>) {
  if args.is_empty() {
    return; 
  }  

  let mut num_args = 1;
  let a = args[0].clone();

  match a.as_ref() {
    "install"  => {
      let (_num_args, install_pkgs, install_flags) =
        parse_install_args(args[1..].to_vec());
      num_args = _num_args;
    },
   _ => panic!("error! "),
  }

  cmd_parse(args[num_args..].to_vec());
}
