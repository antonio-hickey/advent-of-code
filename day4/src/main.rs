fn main() { 
    println!("Part 1 Answer: {}", solution(1));
    println!("Part 2 Answer: {}", solution(2));
}


fn solution(part: i8) ->i32 {
    // Outputs a vector where an element 
    // is 1 if overlap else element is 0
    let elve_pair_is_bad = include_str!("./input.prod")
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(|elf_pair| {
            let (elf_one, elf_two) = elf_pair.split_once(",").unwrap();

            let mut elf_one_sections: Vec<i32> = elf_one.split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            elf_one_sections = (elf_one_sections[0]..elf_one_sections[1] + 1).collect::<Vec<i32>>();

            let mut elf_two_sections: Vec<i32> = elf_two.split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            elf_two_sections = (elf_two_sections[0]..elf_two_sections[1] + 1).collect::<Vec<i32>>();

            if part == 1 {
                // Part one solution (total overlap of sections)
                if elf_one_sections.first() <= elf_two_sections.first() 
                    && elf_one_sections.last() >= elf_two_sections.last() 
                || elf_two_sections.first() <= elf_one_sections.first()
                    && elf_two_sections.last() >= elf_one_sections.last()
                    { return 1 } 
            } else {
                // Part two solution (partial overlap of sections)
                for section in elf_one_sections.iter() {
                    if elf_two_sections.contains(section) {
                        return 1
                    } 
                }
            }
            return 0;
        })
        .collect::<Vec<i32>>();

    elve_pair_is_bad.iter().sum::<i32>()
}

