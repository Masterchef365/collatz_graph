use collatz_dot::*;

fn main() {
    let n = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("100".into())
        .parse()
        .expect("N must be a number");

    let fast_stdout = std::io::BufWriter::new(std::io::stdout());
    write_collatz_graph(fast_stdout, n, "collatz").expect("Stdout io error");
}
