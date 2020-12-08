use reformation::Reformation;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/8.1.txt");
    println!("part01 {}", part01(input));
    println!("part02 {}", part02(input));
}

#[derive(Reformation, Debug, Copy, Clone)]
enum Instr {
    #[reformation("nop {}")]
    Nop(i64),
    #[reformation("acc {}")]
    Acc(i64),
    #[reformation("jmp {}")]
    Jmp(i64),
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
                break;
            }

            self.seen.push(self.pc);

            let i = self.program[self.pc];

            self.pc = match i {
                Instr::Nop(_) => self.pc + 1,
                Instr::Acc(x) => {
                    self.accumulator += x;
                    self.pc + 1
                }
                Instr::Jmp(x) => ((self.pc as i64) + x) as usize,
            };
        }

        (self.pc, self.accumulator)
    }

    // accounts for multiple solutions and returns the highest acc
    fn fix(self) -> i64 {
        let mut last_mut = 0;
        loop {
            let program_copy = self.program.clone();
            let mut fixed_machine = Machine::new(program_copy);

            let (s_pc, s_i) = fixed_machine
                .program
                .iter()
                .enumerate()
                .find(|(pc, i)| {
                    *pc > last_mut
                        && match i {
                            Instr::Jmp(_) => true,
                            Instr::Nop(_) => true,
                            _ => false,
                        }
                })
                .map(|(pc, i)| {
                    (
                        pc,
                        match i {
                            Instr::Jmp(x) => Instr::Nop(*x),
                            Instr::Nop(x) => Instr::Jmp(*x),
                            _ => *i,
                        },
                    )
                })
                .unwrap();

            fixed_machine.program[s_pc] = s_i;
            last_mut = s_pc;

            let acc = fixed_machine.run();
            if acc.0 >= self.program.len() {
                return acc.1;
            }
        }
    }
}

fn part01(s: &str) -> i64 {
    let instructions: Vec<Instr> = s.lines().map(|s| Instr::parse(s).unwrap()).collect();
    let machine = Machine::new(instructions);

    machine.run().1
}

fn part02(s: &str) -> i64 {
    let instructions: Vec<Instr> = s.lines().map(|s| Instr::parse(s).unwrap()).collect();
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
