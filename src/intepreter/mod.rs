mod err;
use err::*;
use std::io::{stdout, Write};

use crate::parse::{
    lexer::{Op, StackOp},
    parser::Func,
};

use super::parse::parser::OpNode;

struct Tape {
    len: usize,
    curr: usize,
    vals: Vec<u32>,
}

impl Tape {
    fn new(size: usize) -> Self {
        Self {
            vals: vec![0; size],
            len: size,
            curr: 0,
        }
    }

    fn curr_val(&self) -> Result<u32, RuntimeError> {
        if self.curr == self.len {
            return Err(RuntimeError::TapeIndexOutOfBounds(self.curr, self.len));
        }
        Ok(self.vals[self.curr])
    }

    fn inc(&mut self) {
        self.vals[self.curr] += 1;
    }
    fn dec(&mut self) {
        self.vals[self.curr] -= 1;
    }

    fn ptr_right(&mut self) -> Result<(), RuntimeError> {
        self.curr += 1;
        Ok(())
    }

    fn ptr_left(&mut self) -> Result<(), RuntimeError> {
        if self.curr == 0 {
            return Err(RuntimeError::TapeIndexCannotBeNegative);
        }
        self.curr -= 1;
        Ok(())
    }

    fn literal(&mut self, lit: &String) {
        for i in (self.curr)..(self.curr + lit.len()).min(self.len) {
            self.vals[i] = lit.chars().nth(i - self.curr).unwrap().into();
        }
    }

    fn dump(&self) -> Result<(), RuntimeError> {
        //print!("Out: {}, {}", unsafe {
        //char::from_u32_unchecked(self.curr_val()?)
        //}, self.curr_val()?);
        print!("{}", unsafe { char::from_u32_unchecked(self.curr_val()?) });
        stdout().flush().unwrap_or_default();
        Ok(())
    }

    fn intake(&mut self) {
        // TODO
    }

    fn bit_not(&mut self) {
        self.set(!self.vals[self.curr]);
    }

    fn set(&mut self, val: u32) {
        self.vals[self.curr] = val;
    }
}

pub struct Runtime {
    stack: Vec<Tape>,
    curr: usize,
    funcs: Vec<Func>,
}

impl Runtime {
    pub fn new(funcs: Vec<Func>) -> Self {
        Self {
            stack: vec![Tape {
                len: 0,
                vals: vec![],
                curr: 0,
            }],
            curr: 0,
            funcs,
        }
    }

    fn curr_tape(&mut self) -> &mut Tape {
        &mut self.stack[self.curr]
    }

    pub fn run_func_with_name(&mut self, name: String) -> Result<(), RuntimeError> {
        for func in self.funcs.iter() {
            if func.name == name {
                #[allow(mutable_borrow_reservation_conflict)]
                self.run_func(&func.clone())?;
                return Ok(());
            }
        }
        Err(RuntimeError::FunctionNotDefined(name))
    }

    fn run_func(&mut self, func: &Func) -> Result<(), RuntimeError> {
        match &func.node {
            OpNode::Root(children) => {
                for child in children {
                    self.run_node(child)?
                }
            }

            // branch should never be reached
            _ => return Err(RuntimeError::Generic),
        };
        Ok(())
    }

    fn run_node(&mut self, node: &OpNode) -> Result<(), RuntimeError> {
        match node {
            OpNode::FuncCall(name) => self.run_func_with_name(name.clone())?,
            OpNode::Operation(op) => self.op(op)?,
            OpNode::Loop(children) => {
                while self.curr_tape().curr_val()? != 0 {
                    for child in children {
                        self.run_node(child)?;
                    }
                }
            }
            OpNode::Root(_) => return Err(RuntimeError::Generic),
        };
        Ok(())
    }

    fn op(&mut self, op: &Op) -> Result<(), RuntimeError> {
        match op {
            // Standard Operations
            Op::Literal(lit) => self.curr_tape().literal(lit),
            Op::Inc => self.curr_tape().inc(),
            Op::Dec => self.curr_tape().dec(),
            Op::PtrRight => self.curr_tape().ptr_right()?,
            Op::PtrLeft => self.curr_tape().ptr_left()?,
            Op::Dump => self.curr_tape().dump()?,
            Op::Take => self.curr_tape().intake(),
            Op::BitNot => self.curr_tape().bit_not(),

            // Stack Operations
            Op::PushNew(size) => self.push_new(*size)?,
            Op::PushOp(op) => match op {
                StackOp::Default => self.push()?,
                StackOp::Add => self.push_op(|curr, next| curr + next)?,
                StackOp::Mul => self.push_op(|curr, next| curr * next)?,
                StackOp::BitAnd => self.push_op(|curr, next| curr & next)?,
                StackOp::Sub => self.push_op(|curr, next| curr - next)?,
                StackOp::Div => self.push_op(|curr, next| curr / next)?,
                StackOp::BitOr => self.push_op(|curr, next| curr | next)?,
                StackOp::BitXor => self.push_op(|curr, next| curr ^ next)?,
                StackOp::Set => self.push_op(|curr, _| curr)?,
            },
            Op::HardPopOp(op) => match op {
                StackOp::Default => self.hard_pop()?,
                StackOp::Add => self.hard_pop_op(|curr, next| curr + next)?,
                StackOp::Mul => self.hard_pop_op(|curr, next| curr * next)?,
                StackOp::BitAnd => self.hard_pop_op(|curr, next| curr & next)?,
                StackOp::Sub => self.hard_pop_op(|curr, next| curr - next)?,
                StackOp::Div => self.hard_pop_op(|curr, next| curr / next)?,
                StackOp::BitOr => self.hard_pop_op(|curr, next| curr | next)?,
                StackOp::BitXor => self.hard_pop_op(|curr, next| curr ^ next)?,
                StackOp::Set => self.hard_pop_op(|curr, _| curr)?,
            },
            Op::PopOp(op) => match op {
                StackOp::Default => self.pop()?,
                StackOp::Add => self.pop_op(|curr, next| curr + next)?,
                StackOp::Mul => self.pop_op(|curr, next| curr * next)?,
                StackOp::BitAnd => self.pop_op(|curr, next| curr & next)?,
                StackOp::Sub => self.pop_op(|curr, next| curr - next)?,
                StackOp::Div => self.pop_op(|curr, next| curr / next)?,
                StackOp::BitOr => self.pop_op(|curr, next| curr | next)?,
                StackOp::BitXor => self.pop_op(|curr, next| curr ^ next)?,
                StackOp::Set => self.pop_op(|curr, _| curr)?,
            },
        }
        Ok(())
    }

    // Stack Operation Specific Funcs
    fn push_new(&mut self, size: usize) -> Result<(), RuntimeError> {
        self.stack.push(Tape::new(size));
        self.push()?;
        Ok(())
    }

    fn push(&mut self) -> Result<(), RuntimeError> {
        self.curr += 1;

        // returns err if pointer exceeds stack length
        if self.curr >= self.stack.len() {
            return Err(RuntimeError::StackIndexOutOfBounds(
                self.curr,
                self.stack.len(),
            ));
        }
        Ok(())
    }

    fn hard_pop(&mut self) -> Result<(), RuntimeError> {
        // deletes current tape then moves down
        self.stack.remove(self.curr);
        self.pop()?;
        Ok(())
    }

    fn pop(&mut self) -> Result<(), RuntimeError> {
        if self.curr == 0 {
            return Err(RuntimeError::StackIndexCannotBeNegative);
        }
        self.curr -= 1;
        Ok(())
    }

    fn pop_op<T>(&mut self, func: T) -> Result<(), RuntimeError>
    where
        T: Fn(u32, u32) -> u32,
    {
        let val1 = self.curr_tape().curr_val()?;
        self.pop()?;
        let curr = self.curr_tape();
        let val2 = curr.curr_val()?;
        curr.set(func(val1, val2));
        Ok(())
    }

    fn hard_pop_op<T>(&mut self, func: T) -> Result<(), RuntimeError>
    where
        T: Fn(u32, u32) -> u32,
    {
        let val1 = self.curr_tape().curr_val()?;
        self.hard_pop()?;
        let curr = self.curr_tape();
        let val2 = curr.curr_val()?;
        curr.set(func(val1, val2));
        Ok(())
    }

    fn push_op<T>(&mut self, func: T) -> Result<(), RuntimeError>
    where
        T: Fn(u32, u32) -> u32,
    {
        let val1 = self.curr_tape().curr_val()?;
        self.push()?;
        let curr = self.curr_tape();
        let val2 = curr.curr_val()?;
        curr.set(func(val1, val2));
        self.pop()?;
        Ok(())
    }
}
