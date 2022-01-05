pub fn collatz(mut x: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        if x == 0 {
            None
        } else {
            let out = x;

            if x & 1 == 0 {
                x /= 2;
            } else {
                x = x * 3 + 1;
            }

            Some(out)
        }
    })
}

pub fn collatz_halt(mut x: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        if x == 0 {
            None
        } else {
            let out = x;

            if x & 1 == 0 {
                x /= 2;
            } else {
                x = x * 3 + 1;
            }

            if out == 1 {
                x = 0;
            }

            Some(out)
        }


    })
}

pub fn write_collatz_graph<F: std::io::Write>(mut f: F, n: u64, name: &str) -> std::io::Result<()> {
    writeln!(f, "strict digraph {} {{", name)?;
    let mut last = n;
    for i in 1..=n {
        for c in collatz_halt(i).skip(1) {
            writeln!(f, "\t{} -> {}", last, c)?;
            last = c;
        }
    }
    writeln!(f, "}}")?;

    Ok(())
}
