//Codifyle, a Rust file testing library
//Copyright © 2015 Brandon Sanderson
//
//Permission is hereby granted, free of charge, to any person obtaining
//a copy of this software and associated documentation files (the "Software"),
//to deal in the Software without restriction, including without limitation
//the rights to use, copy, modify, merge, publish, distribute, sublicense,
//and/or sell copies of the Software, and to permit persons to whom the
//Software is furnished to do so, subject to the following conditions:
//
//The above copyright notice and this permission notice shall be included
//in all copies or substantial portions of the Software.
//
//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
//EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
//OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
//IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
//DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
//TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
//OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::fs::File;
use std::io::Read;
use std::error::Error;

fn read_file_contents(file:&str) -> String {
  let f_opt = File::open(file);

  assert!(f_opt.is_ok(), "Couldn't open {} to check contents", file);

  let mut f = f_opt.unwrap();
  let mut buffer:String = String::new();

  loop {
    let read_result = f.read_to_string(&mut buffer);
    match read_result {
      Ok(0) => break,
      Ok(_) => continue,
      Err(err) =>
        panic!(
          "Failed to read all contents of {}.  Error: {}",
          file,
          err.description()),
    }
  }

  buffer
}

pub fn assert_file_contains_line(file:&str, expected_line:&str){
  let contents = read_file_contents(file);

  for line in contents.lines() {
    if line == expected_line {
      return;
    }
  }

  panic!("Line {} not found in {}", expected_line, file);
}

pub fn assert_file_contents_eq(file:&str, contents:&str){
  let actual_contents = read_file_contents(file);

  assert_eq!(<String as AsRef<str>>::as_ref(&actual_contents), contents);
}

