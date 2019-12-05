use std::convert::TryFrom;

static ADD: i64 = 1;
static MUL: i64 = 2;
static IN: i64 = 3;
static OUT: i64 = 4;
static END: i64 = 99;

static INPUT_VALUE: i64 = 1;

pub fn part_1(input: &str) -> Result<i64, String> {
    let opcodes: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    return solve(opcodes, INPUT_VALUE);
}

fn solve(mut opcodes: Vec<i64>, input: i64) -> Result<i64, String> {
    let mut position = 0;
    let opcode_len = opcodes.len();

    while position < opcode_len {
        match opcodes[position] {
            x if x % 10 == ADD || x % 10 == MUL => {
                let first_param_immediate_mode = x > 100 && x / 100 % 10 > 0;
                let second_param_immediate_mode = x > 1000 && x / 1000 % 10 > 0;

                let x = { 
                    if first_param_immediate_mode { 
                        println!("First param at position {} with immediate mode with value [{}]", position + 1, opcodes[position + 1]);
                        opcodes[position + 1]
                    } else {
                        println!("First param at position {} with position mode with offset [{}] and value [{}]", position + 1, opcodes[position + 1], opcodes[usize::try_from(opcodes[position + 2]).unwrap()]);
                        opcodes[usize::try_from(opcodes[position + 1]).unwrap()]
                    }
                };

                let y = { 
                    if second_param_immediate_mode { 
                        println!("Second param at position {} with immediate mode with value [{}]", position + 2, opcodes[position + 2]);
                        opcodes[position + 2]
                    } else {
                        println!("Second param at position {} with position mode with offset [{}] and value [{}]", position + 2, opcodes[position + 2], opcodes[usize::try_from(opcodes[position + 2]).unwrap()]);
                        opcodes[usize::try_from(opcodes[position + 2]).unwrap()]
                    }
                };

                let res = if x % 10 == ADD {
                    x + y
                } else {
                    x * y
                };

                let res_position = usize::try_from(opcodes[position + 3]).unwrap();
                println!("Current position [{}] points to offset [{}] that has value [{}], saving [{}]..", position + 3, opcodes[position + 3], opcodes[res_position], res);
                opcodes[res_position] = res;
                position += 4;
            }

            x if x == IN => {
                let location = usize::try_from(opcodes[position + 1]).unwrap();
                opcodes[location] = input;
                position += 2;
            }

            x if x == OUT => {
                let location = usize::try_from(opcodes[position + 1]).unwrap();
                println!("OUTPUT!: {}", opcodes[location]);
                position += 2;
            }

            x if x == END => break,
            
            _ => {
                opcodes.iter().for_each(|o| print!("{},", o));
                return Err(format!("Unknown opcode {} at position {}.", opcodes[position], position))
            }

        }
    }

    return Ok(opcodes[0]);
}
