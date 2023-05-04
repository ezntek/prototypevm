#[derive(Clone, Copy)]
enum Comparison {
    Eq(usize, usize),
    Neq(usize, usize),
    Gt(usize, usize),
    Lt(usize, usize),
    Geq(usize, usize),
    Leq(usize, usize),
}

enum Instruction {
    Add { dst: usize, src1: usize, src2: usize },
    Sub { dst: usize, src1: usize, src2: usize },
    Mul { dst: usize, src1: usize, src2: usize },
    Div { dst: usize, src1: usize, src2: usize },
    Copy { dst: usize, src: usize },
    Push { val: i32 },
    Del { src: usize },
    Out { src: usize },
    OutStr { str: &'static str },
    Jump { dst: usize },
    JmpIfZero { dst: usize, src: usize },
    JmpIfNotZero { dst: usize, src: usize},
    JumpCmp { dst: usize, els: usize, cmp: Comparison },
}

struct Vm {
    program: Vec<Instruction>,
    stack: Vec<i32>,
    current_instr: usize,
}

impl Vm {
    fn new(program: Vec<Instruction>) -> Vm {
        Vm {
            stack: Vec::new(),
            program,
            current_instr: 0,
        }
    }
    
    fn run(&mut self) {
        while self.current_instr < self.program.len() {
            match self.program[self.current_instr] {
                Instruction::Add { dst, src1, src2 } => {
                    self.stack[dst] = &self.stack[src1] + &self.stack[src2];
                    self.current_instr += 1;
                }
                Instruction::Sub { dst, src1, src2 } => {
                    self.stack[dst] = &self.stack[src1] - &self.stack[src2];
                    self.current_instr += 1;
                },
                Instruction::Mul { dst, src1, src2 } => {
                    self.stack[dst] = &self.stack[src1] * &self.stack[src2];
                    self.current_instr += 1;
                },
                Instruction::Div { dst, src1, src2 } => {
                    self.stack[dst] = (&self.stack[src1] / &self.stack[src2]) as i32;
                    self.current_instr += 1;
                },
                Instruction::Copy { dst, src } => {
                    self.stack[dst] = self.stack[src].clone();
                    self.current_instr += 1;
                },
                Instruction::Push { val } => {
                    self.stack.push(val);
                    self.current_instr += 1;
                },
                Instruction::Del { src } => {
                    self.stack.remove(src);
                    self.current_instr += 1;
                },
                Instruction::Out { src } => {
                    println!("{}", &self.stack[src]);
                    self.current_instr += 1;
                },
                Instruction::OutStr { str } => {
                    println!("{}", str);
                    self.current_instr += 1;
                } 
                Instruction::Jump { dst } => {
                    self.current_instr = dst;
                },
                Instruction::JmpIfZero { dst, src } => {
                    if src == 0 {
                        self.current_instr = dst;
                    } else {
                        self.current_instr += 1;
                    }
                },
                Instruction::JmpIfNotZero { dst, src } => {
                    if src != 0 {
                        self.current_instr = dst;
                    } else {
                        self.current_instr += 1;
                    }
                },
                Instruction::JumpCmp { dst, els, cmp } => {
                    match cmp {
                        Comparison::Eq(src1, src2) => {
                            if self.stack[src1] == self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Neq(src1, src2) => {
                            if self.stack[src1] != self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Gt(src1, src2) => {
                            if self.stack[src1] > self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Lt(src1, src2) => {
                            if self.stack[src1] < self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Geq(src1, src2) => {
                            if self.stack[src1] >= self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                        Comparison::Leq(src1, src2) => {
                            if self.stack[src1] <= self.stack[src2] {
                                self.current_instr = dst;
                            } else {
                                self.current_instr = els;
                            }
                        },
                    }
                }
            }
        }
    }
}

use Instruction::*;
use Comparison::*;

fn main() {
    let program: Vec<Instruction> = Vec::from([
        Push { val: 4 },
        Push { val: 9 },
        Push { val: 15 },
        Add { dst: 0, src1: 0, src2: 1},
        JumpCmp { dst: 5, els: 10, cmp: Gt(0, 2) },
        // if greater than 15 begin
        OutStr { str: "Greater than 15" },
    ]);

    let mut vm = Vm::new(program);
    vm.run();
}
