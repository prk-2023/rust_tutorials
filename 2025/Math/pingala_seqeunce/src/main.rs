use std::env; // command line argument to read the series 'n' term

fn pingala_series(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![];
    }
    let mut series = vec![0; n]; // generates a vector of n zeros
                                 // set second term F1 = 1 if N >=2
    if n > 1 {
        series[1] = 1;
    }

    // Filling up the series using the Pingala recurrence relation
    // F(i) = F(i-1) + F(i-2)
    // look start at 2 since first two terms are 0,1 which are set
    for i in 2..n {
        series[i] = series[i - 1] + series[i - 2];
    }
    series
}

fn main() {
    // Collect cli arguments
    let args: Vec<String> = env::args().collect();
    //Handle missing argument
    if args.len() < 2 {
        eprintln!("Usage: {} <number_of_terms", args[0]);
        eprintln!("example: {} 10 ", args[0]);
        std::process::exit(1); // exit prog with error code
    }

    // Attempt to parse the second argument (index 1) into uszie (n)
    let n: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            // Non numeric or invalid number input
            eprintln!("Error: the argument '{}' is not valid +ve integer", args[1]);
            // exit with error code
            std::process::exit(1);
        }
    };
    // generate and print the series :
    //
    let pseries = pingala_series(n);

    println!("Pingala Series up to {} terms:", n);
    for (i, term) in pseries.iter().enumerate() {
        println!("Term {}: {}", i + 1, term);
    }
}
