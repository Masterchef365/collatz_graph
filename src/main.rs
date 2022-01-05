use collatz_dot::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let n = args.next().unwrap_or("100".into()).parse().expect("N must be a number");
    write_collatz_graph(std::io::BufWriter::new(std::io::stdout()), n, "collatz").unwrap();

}
