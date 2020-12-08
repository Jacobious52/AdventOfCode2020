use std::str::FromStr;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/8.1.txt");
    println!("part01 {}", part01(input));
    println!("part02 {}", part02(input));
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl FromStr for Instr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        Ok(match tokens.next().unwrap() {
            "nop" => Instr::Nop(tokens.next().unwrap().parse().unwrap()),
            "acc" => Instr::Acc(tokens.next().unwrap().parse().unwrap()),
            "jmp" => Instr::Jmp(tokens.next().unwrap().parse().unwrap()),
            t => panic!("unknown instr {}", t)
        })
    }
}

#[derive(Debug, Default, Clone)]
struct Machine {
    pc: usize,
    program: Vec<Instr>,

    accumulator: i64,

    seen: Vec<usize>,
}

impl Machine {
    fn new(program: Vec<Instr>) -> Self {
        Machine {
            program,
            ..Default::default()
        }
    }

    fn run(mut self) -> (usize, i64) {
        while !self.seen.contains(&self.pc) {
            if self.pc >= self.program.len() {
                break
            }

            self.seen.push(self.pc);

            let i = self.program[self.pc];

            self.pc = match i {
                Instr::Nop(_) => self.pc + 1,
                Instr::Acc(x) => {
                    self.accumulator += x;
                    self.pc + 1
                },
                Instr::Jmp(x) => {
                    ((self.pc as i64) + x) as usize
                }
            };
        }

        (self.pc, self.accumulator)
    }

    // accounts for multiple solutions and returns the highest acc
    fn fix(self) -> i64 {
        let mut found = Vec::new();
        let mut last_mut = 0;
        loop {
            let program_copy = self.program.clone();
            let mut fixed_machine = Machine::new(program_copy);

            let mutate = fixed_machine.program.iter().enumerate().find(|(pc, i)| {
                *pc > last_mut && match i {
                    Instr::Jmp(_) => true,
                    Instr::Nop(_) => true,
                    _ => false,
                }
            }).map(|(pc, i)| {
                (pc, match i {
                    Instr::Jmp(x) => Instr::Nop(*x),
                    Instr::Nop(x) => Instr::Jmp(*x),
                    _ => *i,
                })
            });

            let (s_pc, s_i) = match mutate {
                Some((a, b)) => (a, b),
                None => break,
            };

            dbg!(s_pc, s_i);

            fixed_machine.program[s_pc] = s_i;
            last_mut = s_pc;
            
            let acc = fixed_machine.run();
            if acc.0 >= self.program.len() {
                found.push(acc.1);
            }
        }
        *found.iter().max().unwrap()
    }
}

fn part01(s: &str) -> i64 {
    let instructions: Vec<Instr> = s.lines().map(|s| s.parse().unwrap()).collect();
    let machine = Machine::new(instructions);

    machine.run().1
}

fn part02(s: &str) -> i64 {
    let instructions: Vec<Instr> = s.lines().map(|s| s.parse().unwrap()).collect();
    let machine = Machine::new(instructions);

    machine.fix()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../../inputs/8.1.test.txt");
        assert_eq!(5, part01(input));
    }

    #[test]
    fn test02() {
        let input = include_str!("../../inputs/8.1.test.txt");
        assert_eq!(8, part02(input));
    }
}