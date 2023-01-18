fn main() {
    let logs = xes::read("examples/inputs/example1.xml");
    println!("{:?}", logs);
    xes::write(logs.get(0).unwrap(), "examples/outputs/result1.xml");
}
