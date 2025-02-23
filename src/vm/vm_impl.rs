use std::collections::HashMap;

use crate::common::lexer::lexer_impl::Lexer;

use super::{
    chunk::{value_equal, OpCode, Value},
    compiler::Compiler,
    object::FunctionType,
};

#[derive(Debug)]
pub struct VirtualMachine<'a> {
    pc: usize,
    pub stack: Vec<Value>,
    compiler: &'a mut Compiler<'a>,
    pub globals: HashMap<String, Value>,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(compiler: &'a mut Compiler<'a>) -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            stack: vec![],
            compiler,
            globals: HashMap::new(),
        }
    }

    pub fn new_with_globals(
        compiler: &'a mut Compiler<'a>,
        globals: HashMap<String, Value>,
    ) -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            stack: vec![],
            compiler,
            globals, // Existing globals
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        loop {
            // Gets next OpCode using current PC
            let pc = self.pc;
            let chunk = self.compiler.current_chunk();
            let Some(instruction) = chunk.get(pc) else {
                return InterpretResult::Ok;
            };

            self.pc += 1; // Increases current PC for next instruction
            match instruction {
                OpCode::Constant(index) => {
                    let Some(constant) = chunk.get_constant(*index).cloned() else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(constant);
                }
                OpCode::True => self.stack.push(Value::Boolean(true)),
                OpCode::False => self.stack.push(Value::Boolean(false)),
                OpCode::Null => self.stack.push(Value::Null),
                OpCode::Not => {
                    if let Some(value) = self.stack.last_mut() {
                        *value = Value::Boolean(is_falsey(value));
                    } else {
                        return InterpretResult::RuntimeError;
                    }
                }
                OpCode::Equal => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(Value::Boolean(value_equal(a, b))),
                    (_, _) => return InterpretResult::RuntimeError,
                },
                OpCode::NotEqual => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(Value::Boolean(!value_equal(a, b))),
                    (_, _) => return InterpretResult::RuntimeError,
                },
                OpCode::Greater => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(first_value)), Some(Value::Number(second_value))) => {
                        self.stack.push(Value::Boolean(second_value > first_value));
                    }
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::GreaterEqual => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(first_value)), Some(Value::Number(second_value))) => {
                        self.stack.push(Value::Boolean(second_value >= first_value));
                    }
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Less => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(first_value)), Some(Value::Number(second_value))) => {
                        self.stack.push(Value::Boolean(second_value < first_value));
                    }
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::LessEqual => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(first_value)), Some(Value::Number(second_value))) => {
                        self.stack.push(Value::Boolean(second_value <= first_value));
                    }
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Negate => {
                    if let Some(value) = self.stack.last_mut() {
                        match value {
                            Value::Number(n) => *value = Value::Number(-*n),
                            _ => return InterpretResult::RuntimeError,
                        }
                    } else {
                        return InterpretResult::RuntimeError;
                    }
                }
                OpCode::Add => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(first_value)), Some(Value::Number(second_value))) => {
                        self.stack.push(Value::Number(first_value + second_value));
                    }
                    (Some(Value::String(first_value)), Some(Value::String(second_value))) => {
                        self.stack
                            .push(Value::String(format!("{}{}", second_value, first_value)));
                    }
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Subtract => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(first_value)), Some(Value::Number(second_value))) => {
                        self.stack.push(Value::Number(second_value - first_value));
                    }
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Multiply => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(first_value)), Some(Value::Number(second_value))) => {
                        self.stack.push(Value::Number(first_value * second_value));
                    }
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Divide => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(first_value)), Some(Value::Number(second_value))) => {
                        if first_value == 0.0 {
                            return InterpretResult::RuntimeError;
                        }
                        self.stack.push(Value::Number(second_value / first_value));
                    }
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::DefineGlobal(index) => {
                    match (chunk.get_constant(*index), self.stack.last()) {
                        (Some(Value::String(name)), Some(last)) => {
                            self.globals.insert(name.clone(), last.clone());
                        }
                        (_, _) => return InterpretResult::RuntimeError,
                    }
                }
                OpCode::GetGlobal(index) => {
                    let Some(Value::String(name)) = chunk.get_constant(*index) else {
                        return InterpretResult::RuntimeError;
                    };

                    match self.globals.get(name) {
                        Some(value) => self.stack.push(value.clone()),
                        None => return InterpretResult::RuntimeError,
                    };
                }
                OpCode::SetGlobal(index) => {
                    let Some(Value::String(name)) = chunk.get_constant(*index) else {
                        return InterpretResult::RuntimeError;
                    };

                    match self.stack.pop() {
                        Some(value) => self.globals.insert(name.to_string(), value),
                        None => return InterpretResult::RuntimeError,
                    };
                }
                OpCode::GetLocal(index) => {
                    // -1 because first locals slot is for vm
                    let Some(value) = self.stack.get(*index - 1) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(value.clone());
                }
                OpCode::SetLocal(index) => {
                    self.stack[*index] = self.stack[self.stack.len() - 1].clone();
                }
                OpCode::JumpIfFalse(target) => {
                    let last_index = self.stack.len() - 1;

                    if is_falsey(&self.stack[last_index]) {
                        self.pc += target;
                    }
                }
                OpCode::Jump(target) => {
                    self.pc += target;
                }
                OpCode::Loop(target) => {
                    self.pc -= *target;
                }
                OpCode::Return => {
                    println!("{}", self.stack.pop().unwrap_or(Value::Null));
                    return InterpretResult::Ok;
                }
            }
        }
    }
}

fn is_falsey(value: &Value) -> bool {
    match value {
        Value::Boolean(bool) => !bool,
        Value::Null => true,
        _ => false,
    }
}

pub fn compile_and_run(input: String) {
    let mut lexer = Lexer::new(&input);
    let mut compiler = Compiler::new(&mut lexer, FunctionType::Script);

    if matches!(compiler.compile(), InterpretResult::CompileError) {
        println!("compiler has {} errors", compiler.errors.len());
        for error in compiler.errors {
            println!("compiler error: {}", error);
        }
        return;
    }

    let mut vm = VirtualMachine::new(&mut compiler);

    vm.interpret();
}
