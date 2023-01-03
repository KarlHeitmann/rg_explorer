mod nodes;
mod io;

fn run(results: Vec<&str>) {
    let parsed_result = nodes::Nodes::new(results);
    println!("{}", parsed_result);
}


fn main() {
    println!("Hello, world!");
    let results = io::run_command();
    // let parsed_result = result;
    run(results.split("\n").collect::<Vec<&str>>());
    // let r = rg_wrapper::RgWrapper::new(results.split("\n").collect::<Vec<&str>>());


}
