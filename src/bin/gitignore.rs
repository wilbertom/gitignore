
extern crate curl;

use curl::http;
use std::io::File;
use std::os;


fn get(u: &str) -> http::Response {
  http::handle().get(u).exec().unwrap()

}

fn get_gitignore(filename: &str) -> http::Response {
  let url = format!("https://raw.githubusercontent.com/github/gitignore/master/{}", filename);
  get(url.as_slice())

}

fn main() {
  let args = os::args();

  match args.len() {
    1 => {
      println!("Usage: {} <name>", args[0]);
      println!("where name can be any of the files located at");
      println!("https://github.com/github/gitignore");
      println!("without the .gitignore extension");

      return
    },
    _ => {
      let s = format!("{}.gitignore", args[1]);
      let filename = s.as_slice();
      let resp = get_gitignore(filename);
      let gitingore_string = resp.get_body();

      let path = Path::new(".gitignore");
      let mut file = File::create(&path);

      file.write(gitingore_string);
    }
  }
}
