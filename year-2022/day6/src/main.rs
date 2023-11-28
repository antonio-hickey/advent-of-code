fn main() {
    let stream = include_str!("./input.prod").chars()
        .filter(|&c| c != '\n' as char)
        .collect::<Vec<char>>();

    println!("Part 1 Answer: {}", solution(&stream, 1));
    println!("Part 2 Answer: {}", solution(&stream, 2));
}

fn solution(stream: &Vec<char>, part: i8) -> usize {
    // Bruteforce solution (come back to refactor)
    

    // The solution doesn't have to change just the size of the window
    // we use to iterate and search over.
    let n: usize = if part == 1 {
        // Part one looks for the start of a packet in the datastream
        // which is indicated by 4 unique characters.
        4
    } else {
        // Part two looks for the start of a message in the datastream 
        // which is indicated 14 unique characters.
        14
    };


    'next_window: for (idx, window) in stream.windows(n).enumerate() {
        for i in 0..(n - 1) {
            for j in i + 1..n {
                if window[i] == window[j] {
                    continue 'next_window
                }
            }
        }

        return idx + n;
    }
    
    // Should only be hit if Advent of Code gave bad input
    return 0;
}
