/*
  - if the desired function is nested in more
    than one module, it's conventional to bring
    the parent module into scope rather then
    the function
  - env:args() clearly indicate that the args
    function is definitely not defined locally
*/
use std::env;
use std::process;

use rust_minigrep::Config;

/*
  - you can't test the main function directly
  - so the code remains in main.rs should be
    small enough to verify its correctness
    by reading it
*/
fn main() {
  let args: Vec<String> = env::args().collect();

  /*
    - you unwrap the OK variant and return
      the value in it hence the term unwrap
    - the unwrap_or_else method takes in a closure
      (anonymous function)
  */

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  /*
    - use "if let" instead of "match" if you
      don't want to handle cases other
      than errors
    - in this case we are only interested in
      handling the errors if there is any
    - if `let` destructures the returned value
      from run into `Err(e)`, evaluate the
      block (`{}`).
  */
  if let Err(e) = rust_minigrep::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  };
}
