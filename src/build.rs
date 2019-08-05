use std::process::Command;
use std::env::set_current_dir;
use std::env::current_dir;
use std::path::Path;
use std::fs;

fn ls(tmpdir: String) {
  println!("files:");
  for f in fs::read_dir(Path::new(&tmpdir)).unwrap() {
    println!("{}", f.unwrap().path().display())
  }
}

fn git_clone(tmpdir: String, repo : String) {
  Command::new("git").arg("clone").arg(repo);
}

pub fn fetch(tmpdir: String, pkg : String) {
  let p = Path::new(&pkg);  
  if p.extension().unwrap().to_str() == Some("git") {
    git_clone(tmpdir, pkg);
  }
  else {
    ();
  }
}

fn make_temp_dir() -> String {
  let stdout = Command::new("mktemp").arg("-d").output().expect("oops").stdout;
  let path_str : String = String::from_utf8_lossy(&stdout).to_string();
  return path_str;
}

pub fn build_rpm() {
  let cwd = current_dir().unwrap(); 
  let tmpdir = make_temp_dir();
  let tmp = Path::new(&tmpdir);

  println!("{}", tmpdir);
  fetch(tmpdir.clone(), String::from("https://github.com/dbenoit17/icky-pkg-index.git"));

 
}
