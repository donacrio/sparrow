const CAPACITY: usize = 1000;
const ADDR: &str = "127.0.0.1:3000";

fn main() {
  match sparrow_core::run(CAPACITY, ADDR) {
    Ok(_) => std::process::exit(0),
    Err(_) => {
      //TODO: Implement Display for errors
      eprintln!("an error occured");
      std::process::exit(1)
    }
  };
}
