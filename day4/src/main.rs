fn main() { 
    // Vector where the value is 1 if bad pair else is 0
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

            if elf_one_sections.first() <= elf_two_sections.first() 
                && elf_one_sections.last() >= elf_two_sections.last() 
            || elf_two_sections.first() <= elf_one_sections.first()
                && elf_two_sections.last() >= elf_one_sections.last()
            { return 1 };

            return 0;
        })
        .collect::<Vec<i32>>();
    
    println!("Part 1 Answer: {}", elve_pair_is_bad.iter().sum::<i32>());
}
