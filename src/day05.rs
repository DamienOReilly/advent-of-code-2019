static ADD: i64 = 1;
static MUL: i64 = 2;
static IN: i64  = 3;
static OUT: i64 = 4;
static JNT: i64 = 5;
static JNF: i64 = 6;
static LT: i64  = 7;
static EQ: i64  = 8;
static END: i64 = 99;

pub fn part_1(input: &str) -> Result<i64, String> {
    let mut opcodes: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    solve(&mut opcodes, 1)
}

pub fn part_2(input: &str) -> Result<i64, String> {
    let mut opcodes: Vec<i64> = input.split(",").map(|s| s.parse().unwrap()).collect();
    solve(&mut opcodes, 5)
}

fn get_parameters(immediate_mode: bool, position: usize, opcodes: Vec<i64>) -> i64 { 
        if immediate_mode { 
            opcodes[position]
        } else {
            opcodes[opcodes[position] as usize]
        }
}

fn write_value(position: usize, value: i64, opcodes: &mut Vec<i64>) -> usize {
    let res_position = opcodes[position] as usize;
    opcodes[res_position] = value;
    4
}

fn solve(opcodes: &mut Vec<i64>, input: i64) -> Result<i64, String> {
    let mut position = 0;
    let opcode_len = opcodes.len();
    let mut diagnostic_codes = Vec::new();

    while position < opcode_len {
        let first_param_immediate_mode  = opcodes[position] > 100  && opcodes[position] / 100  % 10 > 0;
        let second_param_immediate_mode = opcodes[position] > 1000 && opcodes[position] / 1000 % 10 > 0;
        match opcodes[position] {
            op if op % 10 == ADD || op % 10 == MUL => {
                let x = get_parameters(first_param_immediate_mode, position + 1, opcodes.clone());
                let y = get_parameters(second_param_immediate_mode, position + 2, opcodes.clone());

                let res = if op % 10 == ADD {
                    x + y
                } else {
                    x * y
                };
                position += write_value(position + 3, res, opcodes);
            }

            op if op == IN => {
                let location = opcodes[position + 1] as usize;
                opcodes[location] = input;
                position += 2;
            }

            op if op % 10 == OUT => {
                let output = { 
                    if first_param_immediate_mode { 
                        opcodes[position + 1]
                    } else {
                        opcodes[opcodes[position + 1] as usize]
                    }
                };                
                diagnostic_codes.push(output);
                position += 2;
            }

            op if op % 10 == JNT => {
                let x = get_parameters(first_param_immediate_mode, position + 1, opcodes.clone());
                let y = get_parameters(second_param_immediate_mode, position + 2, opcodes.clone());

                if x != 0 {
                    position = y as usize;
                } else {
                    position += 3;
                }
            }

            op if op % 10 == JNF => {
                let x = get_parameters(first_param_immediate_mode, position + 1, opcodes.clone());
                let y = get_parameters(second_param_immediate_mode, position + 2, opcodes.clone());

                if x == 0 {
                    position = y as usize;
                } else {
                    position += 3;
                }
            }

            op if op % 10 == LT => {
                let x = get_parameters(first_param_immediate_mode, position + 1, opcodes.clone());
                let y = get_parameters(second_param_immediate_mode, position + 2, opcodes.clone());

                let res = if x < y {
                    1
                } else {
                    0
                };
                position += write_value(position + 3, res, opcodes);
            }

            op if op % 10 == EQ => {
                let x = get_parameters(first_param_immediate_mode, position + 1, opcodes.clone());
                let y = get_parameters(second_param_immediate_mode, position + 2, opcodes.clone());

                let res = if x == y {
                    1
                } else {
                    0
                };
                position += write_value(position + 3, res, opcodes);
            }

            op if op == END => break,
            _ => return Err(format!("Unknown opcode {} at position {}", opcodes[position], position))
        }
    }

    match diagnostic_codes.last() {
        Some(n) => Ok(*n),
        None    => Err("Failed to solve puzzle!".into())        
    }
}
