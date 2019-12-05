use std::convert::TryFrom;

static ADD: i64 = 1;
static MUL: i64 = 2;
static IN: i64 = 3;
static OUT: i64 = 4;
static END: i64 = 99;

static INPUT_VALUE: i64 = 1;

pub fn part_1(input: &str) {
    let opcodes: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    solve(opcodes, INPUT_VALUE);
}

fn solve(mut opcodes: Vec<i64>, input: i64) {
    let mut position = 0;
    let opcode_len = opcodes.len();

    while position < opcode_len {
        match opcodes[position] {
            op if op % 10 == ADD || op % 10 == MUL => {
                let first_param_immediate_mode  = op > 100  && op / 100  % 10 > 0;
                let second_param_immediate_mode = op > 1000 && op / 1000 % 10 > 0;
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

                let res = if op % 10 == ADD {
                    println!("adding");
                    x + y
                } else {
                    println!("muliplying");
                    x * y
                };

                let res_position = usize::try_from(opcodes[position + 3]).unwrap();
                println!("Current position [{}] points to offset [{}] that has value [{}], saving [{}]..", position + 3, opcodes[position + 3], opcodes[res_position], res);
                opcodes[res_position] = res;
                position += 4;
            }

            op if op == IN => {
                let location = usize::try_from(opcodes[position + 1]).unwrap();
                opcodes[location] = input;
                position += 2;
            }

            op if op % 10 == OUT => {
                let first_param_immediate_mode = op > 100 && op / 100 % 10 > 0;                
                let output = { 
                    if first_param_immediate_mode { 
                        opcodes[position + 1]
                    } else {
                        opcodes[usize::try_from(opcodes[position + 1]).unwrap()]
                    }
                };                
                println!("OUTPUT!: {}", output);
                position += 2;
            }

            op if op == END => break,
            
            _ => {
                opcodes.iter().for_each(|o| print!("{},", o));
                println!("Unknown opcode {} at position {}.", opcodes[position], position);
                break;
            }

        }

    }
    
}
