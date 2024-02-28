use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    value: usize,
}

fn fib_recursive(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn fib_iter(n: usize) -> usize {
    let mut memo: Vec<usize> = vec![0, 1];

    if n < 2 {
        return memo[n];
    }

    for i in 2..=n {
        let next_value = memo[i - 1] + memo[i - 2];
        memo.push(next_value);
    }

    memo[n]
}

fn main() {
    let value = Args::parse().value;

    let result_rec = fib_recursive(value);

    println!("recursive result {}", result_rec);

    let result_iter = fib_iter(value);

    println!("iterative result {}", result_iter);
}

#[cfg(test)]
mod tests {
    use crate::{fib_iter, fib_recursive};

    #[test]
    fn are_they_equal() {
        let it_result = fib_iter(13);
        let rec_result = fib_recursive(13);
        assert_eq!(it_result, rec_result);
    }
}
