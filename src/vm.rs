use crate::op_code::{OpCode, OpCode::*};
use crate::chunk::Chunk;
use crate::compiler::compile;
use crate::debug::disassemble_chunk;
use crate::vm::InterpretOk::INTERPRET_OK;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum InterpretError {
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum InterpretOk{
    INTERPRET_CONTINUE,
    INTERPRET_OK,
}

const STACK_MAX: usize = 256;

#[allow(non_snake_case)]
#[derive(Default)]
pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    // instruction pointer, points at bytecode about to be executed
    pub stack: Vec<f32>,
}

impl VM {
    pub const DEBUG_EXECUTION_TRACING: bool = true;

    pub fn interpret(&mut self, source: &String) -> Result<Option<f32>, InterpretError> {
        let result = compile(source);
        match result {
            Err(_) => {
                return Err(InterpretError::INTERPRET_COMPILE_ERROR)
            },
            Ok(chunk) => {
                self.chunk = chunk;
                self.ip = 0; // Q

                let result = self.run();

                return result;
            },
        }
    }

    fn push(&mut self, value: f32) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> f32 {
        self.stack.pop().unwrap()
    }

    //Q: what happens when there are multiple chunks?
    pub fn run(&mut self) -> Result<Option<f32>, InterpretError> {
        // if debug flag enabled, print each instruction before execution
        if VM::DEBUG_EXECUTION_TRACING {
            println!("           ");
            for val in self.stack.iter() {
                println!("[{}]", val);
            }
            disassemble_chunk(&self.chunk, "chunk");
            println!();
        }

        loop {
            let instruction = self.read_byte();
            let mut intepret_ok: bool = false;
            match instruction {
                OP_RETURN => {
                    if let Some(v) = self.stack.pop() {
                        println!("chunk result: {}", v);
                        return Ok(Some(v));
                    } else {
                        println!("Stack is empty, nothing to pop");
                        return Ok(None);
                    }
                    // intepret_ok = true;
                }
                OP_CONSTANT(c) => {
                    let c = c.clone();
                    self.stack.push(c);
                }
                OP_NEGATE => {
                    let pop_val = self.stack.pop().unwrap();
                    self.stack.push(
                        pop_val * -1., // negating
                    );
                }
                OP_ADD => {
                    binary_operator(self, '+');
                }
                OP_SUBTRACT => {
                    binary_operator(self, '-');
                }
                OP_MULTIPLY => {
                    binary_operator(self, '*');
                }
                OP_DIVIDE => {
                    binary_operator(self, '/');
                }
                OP_CONSTANT_LONG(_) => {
                    unimplemented!()
                }
                OP_DEBUG => {
                    unimplemented!()
                }
            };

            // if intepret_ok {
            //     return Ok(INTERPRET_OK);
            // }
        }
    }

    fn read_byte(&mut self) -> &OpCode {
        let instruction = self.chunk.code.get(self.ip);
        match instruction {
            None => {
                unimplemented!()
            }
            Some(instruction) => {
                self.ip += 1;
                return instruction;
            }
        }
    }

    pub fn with_chunk(mut self, chunk: Chunk) -> Self {
        self.chunk = chunk;
        return self;
    }
}

fn binary_operator(vm: &mut VM, op: char) {
    let b: f32 = vm.stack.pop().unwrap();
    let a: f32 = vm.stack.pop().unwrap();
    match op {
        '+' => vm.stack.push(a + b),
        '-' => vm.stack.push(a - b),
        '/' => vm.stack.push(a / b),
        '*' => vm.stack.push(a * b),
        _ => {
            println!("invalid operation {}", op)
        }
    }
}
