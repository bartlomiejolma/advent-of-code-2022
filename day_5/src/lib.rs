use regex::Regex;

fn decode_message_generic(
    text: &str,
    execute_command_fn: fn(Stacks, &Command) -> Stacks,
) -> String {
    let mut elements = text.split("\n\n");
    let stacks_text = elements.next().unwrap();
    let commands_text = elements.next().unwrap();

    let mut stacks = Stacks::parse(stacks_text);

    for command_line in commands_text.lines() {
        let command = Command::parse(command_line);
        println!("{:?} {:?}", stacks, command);

        stacks = execute_command_fn(stacks, &command);
        // println!("{:?}", stacks);
    }

    Stacks::result(&stacks)
}

pub fn decode_message(text: &str) -> String {
    decode_message_generic(text, Stacks::execute_command)
}

pub fn decode_message_9001(text: &str) -> String {
    decode_message_generic(text, Stacks::execute_command_9001)
}

#[derive(Debug, PartialEq)]
struct Command {
    from: usize,
    to: usize,
    count: usize,
}

impl Command {
    fn parse(line: &str) -> Command {
        let re = Regex::new(r#"move (?P<count>\d*) from (?P<from>\d*) to (?P<to>\d*)"#).unwrap();
        let captures = re.captures(line).unwrap();
        Command {
            from: captures
                .name("from")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            to: captures
                .name("to")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            count: captures
                .name("count")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn parse(text: &str) -> Stacks {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        text.lines().for_each(|line| {
            Vec::from_iter(line.chars().into_iter())
                .chunks(4)
                .enumerate()
                .for_each(|(index, block)| {
                    // println!(
                    //     "{:?} {:?} {:?} {:?}",
                    //     block,
                    //     index,
                    //     line,
                    //     Vec::from_iter(line.chars().into_iter())
                    // );

                    if stacks.len() < index + 1 {
                        stacks.push(Vec::new())
                    }
                    match block.get(0) {
                        Some('[') => stacks[index].push(block.get(1).unwrap().clone()),
                        _ => (),
                    }
                })
        });
        let reversed = stacks
            .into_iter()
            .map(|mut stack| {
                stack.reverse();
                stack
            })
            .collect();
        Stacks { stacks: reversed }
    }

    fn execute_command(Self { mut stacks }: Self, command: &Command) -> Stacks {
        for _i in 0..command.count {
            // println!(
            //     "{:?} {:?} {:?} {:?}",
            //     stacks,
            //     command,
            //     i,
            //     [0..command.count]
            // );
            let element = stacks[command.from - 1].pop().unwrap().clone();
            stacks[command.to - 1].push(element)
        }

        Stacks { stacks: stacks }
    }

    fn execute_command_9001(Self { mut stacks }: Self, command: &Command) -> Stacks {
        let mut slice: Vec<char> = (0..command.count)
            .into_iter()
            .map(|_| stacks[command.from - 1].pop().unwrap().clone())
            .collect();
        slice.reverse();
        stacks[command.to - 1].extend(slice);
        Stacks { stacks: stacks }
    }

    fn result(&self) -> String {
        let message: &String = &(&self.stacks)
            .into_iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect();
        message.to_owned()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "move 1 from 2 to 1";

        let expected_command = Command {
            count: 1,
            from: 2,
            to: 1,
        };

        let actual = Command::parse(line);

        assert_eq!(actual, expected_command)
    }

    #[test]
    fn test_parse_stacks() {
        let text = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3";

        let expected = Stacks {
            stacks: Vec::from([
                Vec::from(['Z', 'N']),
                Vec::from(['M', 'C', 'D']),
                Vec::from(['P']),
            ]),
        };

        let actual = Stacks::parse(text);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_execute_command() {
        let command = Command {
            count: 1,
            from: 2,
            to: 1,
        };

        let initial = Stacks {
            stacks: Vec::from([
                Vec::from(['Z', 'N']),
                Vec::from(['M', 'C', 'D']),
                Vec::from(['P']),
            ]),
        };

        let expected = Stacks {
            stacks: Vec::from([
                Vec::from(['Z', 'N', 'D']),
                Vec::from(['M', 'C']),
                Vec::from(['P']),
            ]),
        };
        let actual = Stacks::execute_command(initial, &command);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_result() {
        let command = Command {
            count: 1,
            from: 2,
            to: 1,
        };

        let initial = Stacks {
            stacks: Vec::from([
                Vec::from(['Z', 'N']),
                Vec::from(['M', 'C', 'D']),
                Vec::from(['P']),
            ]),
        };

        let expected = "NDP";
        let actual = Stacks::result(&initial);

        assert_eq!(actual, expected)
    }
    #[test]
    fn solve() {
        let text = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let expected_message = "CMZ";

        let actual_message = decode_message(text);

        assert_eq!(expected_message, actual_message);
    }

    #[test]
    fn solve_2() {
        let text = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let expected_message = "MCD";

        let actual_message = decode_message_9001(text);

        assert_eq!(expected_message, actual_message);
    }
}
