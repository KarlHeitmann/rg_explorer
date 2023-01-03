mod nodes;
mod io;

fn run(results: Vec<&str>) {
    let parsed_result = nodes::Nodes::new(results);
    println!("{}", parsed_result);
}

fn main() {
    println!("Hello, world!");
    let results = io::run_command();
    run(results.split("\n").collect::<Vec<&str>>());
}

