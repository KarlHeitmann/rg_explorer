mod io;

fn main() {
    println!("Hello, world!");
    let result = io::run_command();
    println!("{}", result);

}
