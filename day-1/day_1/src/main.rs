use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input = parse_input(contents);
    // println!("{:?}", input);
    println!("{:?}", input.max_total(1));
    println!("{:?}", input.max_total(3));

}

#[derive(Debug)]
struct ElfCalories {
    food: Vec<u32>,
}

impl ElfCalories {
    fn total(&self) -> u32 {
        let total_value = self
            .food
            .iter()
            .map(Clone::clone)
            .reduce(u32::wrapping_add)
            .unwrap();
        total_value
    }
}
#[derive(Debug)]
struct Input {
    elf_calories: Vec<ElfCalories>,
}

impl Input {
    fn max(&self, n: usize) -> Vec<u32> {
        let mut callories = self
            .elf_calories
            .iter()
            .map(ElfCalories::total)
            .collect::<Vec<u32>>();
        callories.sort_by(|a, b| b.cmp(a));
        callories[0..n].to_vec()
    }

    fn max_total(&self, n: usize) -> u32 {
        self.max(n).iter().map(Clone::clone).reduce(u32::wrapping_add).unwrap()
    }
}

fn parse_elf_input(elf_input: &str) -> ElfCalories {
    ElfCalories {
        food: elf_input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect(),
    }
}

fn parse_input(input_text: String) -> Input {
    Input {
        elf_calories: input_text.split("\n\n").map(parse_elf_input).collect(),
    }
}
