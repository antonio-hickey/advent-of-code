fn main() { 
    println!("Part 1 Answer: {}", solution(1));
    println!("Part 2 Answer: {}", solution(2));
}

fn get_section(elf: &str) -> Vec<i32> {
    let sections: Vec<i32> = elf.split("-")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    return (
        sections[0]..sections[1] + 1
    ).collect::<Vec<i32>>();
}

fn solution(part: i8) ->i32 {
    return include_str!("./input.prod")
        .split("\n").filter(|&x| !x.is_empty())
        .map(|elf_pair| {
            let (elf_one, elf_two) = elf_pair.split_once(",").unwrap();

            let elf_one_sections: Vec<i32> = get_section(elf_one);
            let elf_two_sections: Vec<i32> = get_section(elf_two);

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
        }).sum::<i32>();
}

