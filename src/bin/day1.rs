fn main() {
    let input = include_str!("day1.txt");
    let mut sums: Vec<i32> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&elf_section| {
            elf_section
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .filter_map(|&cals_str| cals_str.parse::<i32>().ok())
                .sum()
        })
        .collect();

    let max = sums.iter().max().unwrap();
    println!("Maximum is {}", max);

    sums.sort();
    let sum_top_three: i32 = sums.iter().rev().take(3).sum();
    println!("Cals of maximum three are {}", sum_top_three);
}
