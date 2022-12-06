fn main() {
    let stream = include_str!("./input.prod").chars()
        .filter(|&c| c != '\n' as char)
        .collect::<Vec<char>>();

    println!("Part 1 Answer: {}", solution(&stream));
    println!("Part 2 Answer: UNSOLVED");
}

fn solution(stream: &Vec<char>) -> usize {
    // Bruteforce solution (come back to refactor)
    'next_window: for (idx, window) in stream.windows(4).enumerate() {
        for i in 0..3 {
            for j in i + 1..4 {
                if window[i] == window[j] {
                    continue 'next_window
                }
            }
        }

        return idx + 4;
    }
    
    // Should only be hit if Advent of Code gave bad input
    return 0;
}
