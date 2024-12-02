use lending_iterator::LendingIterator;

fn main() {
    let input = include_bytes!("../../data/input.txt");
    let val = input
        .split(|&b| b == b'\n')
        .take_while(|&line| !line.is_empty())
        .map(|line| {
            // Get our list of values which are separated by white space
            // Use an i32 because there can be negative numbers
            let mut vals = line
                .split(|&b| b == b' ')
                .filter_map(atoi::atoi::<i32>)
                .collect::<Vec<_>>();

            (0..vals.len())
                // Iteratre backwards through our iterator
                .rev()
                .map(|idx| {
                    // Get a mutable window of the current size
                    // Each iteration, we will see 1 less entry than we did before
                    lending_iterator::windows_mut(&mut vals[..=idx]).for_each(|w: &mut [_; 2]| 
                        // Update our left most number to be the difference between
                        // the two entries
                        w[0] = w[1] - w[0]
                    );

                    vals[idx]
                })
                // Since we are replacing each entry in the list, we can sum up
                // every entry (0s are effectively ignored) to get our total.
                // We see one less entry on every iteration so the previous iterations
                // value will be right most
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Total: {}", val);
}
