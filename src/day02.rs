static ADD: usize = 1;
static MUL: usize = 2;
static END: usize = 99;

pub fn part_1(input: &str) -> Result<usize, String> {
    let opcodes: Vec<usize> = input.split(",").map(|s| s.parse().unwrap()).collect();
    return solve(opcodes, 12, 2);
}

pub fn part_2(input: &str) -> Result<usize, String> {
    let opcodes: Vec<usize> = input.split(",").map(|s| s.parse().unwrap()).collect();
    for n in 0..99 {
        for v in 0..99 {
            let copy = opcodes.clone();
            let attempt = solve(copy, n, v);

            match attempt {
                Ok(res) if res == 19690720 => { return Ok(100 * n + v); }
                _ => { continue; }
            }
        }
    }

    return Err("Cannot solve!".to_string());
}

fn solve(mut opcodes: Vec<usize>, noun: usize, verb: usize) -> Result<usize, String> {
    opcodes[1] = noun;
    opcodes[2] = verb;

    let mut position = 0;
    let opcode_len = opcodes.len();

    while position < opcode_len {
        match opcodes[position] {
            x if x == ADD || x == MUL => {
                let sum = if x == ADD {
                    opcodes[opcodes[position + 1]] + opcodes[opcodes[position + 2]]
                } else {
                    opcodes[opcodes[position + 1]] * opcodes[opcodes[position + 2]]
                };

                let res_position = opcodes[position + 3];
                opcodes[res_position] = sum;
                position += 4;
            }

            x if x == END => break,
            
            _ => return Err(format!("Unknown opcode {} at position {}", opcodes[position], position))
        }
    }

    return Ok(opcodes[0]);
}
