use crate::{
    context::Context,
    values::{number::Number, value::Value},
};
use glang_attributes::{Position, StandardError};
use std::{cell::RefCell, iter::zip, rc::Rc};

#[derive(Debug, Clone)]
pub struct List {
    pub elements: Vec<Rc<RefCell<Value>>>,
    pub context: Option<Rc<RefCell<Context>>>,
    pub is_const: bool,
    pub pos_start: Option<Rc<Position>>,
    pub pos_end: Option<Rc<Position>>,
}

impl List {
    pub fn new(elements: Vec<Rc<RefCell<Value>>>) -> Self {
        Self {
            elements,
            context: None,
            is_const: false,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn from(elements: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value::ListValue(List::new(elements))))
    }

    pub fn perform_operation(
        &mut self,
        operator: &str,
        other: Rc<RefCell<Value>>,
    ) -> Result<Rc<RefCell<Value>>, StandardError> {
        if self.is_const {
            return Err(StandardError::new(
                "cannot change a constant value",
                self.pos_start.as_ref().unwrap().clone(),
                self.pos_end.as_ref().unwrap().clone(),
                None,
            ));
        }

        if other.borrow().is_const() {
            return Err(StandardError::new(
                "cannot change a constant value",
                other.borrow().position_start().unwrap(),
                other.borrow().position_end().unwrap(),
                None,
            ));
        }

        if operator == "*" {
            return Ok(self.push(other.clone()));
        }

        match *other.borrow_mut() {
            Value::ListValue(ref value) => match operator {
                "+" => Ok(self.append(&mut value.elements.clone())),
                "==" => {
                    let mut is_eq = Number::null_value();
                    is_eq.borrow_mut().set_context(self.context.clone());

                    for (a, b) in zip(&self.elements, &value.elements) {
                        let result = a.borrow_mut().perform_operation("==", b.clone());

                        if result.is_err() {
                            return Err(result.err().unwrap());
                        }

                        is_eq = result.ok().unwrap();
                    }

                    Ok(is_eq)
                }
                "!=" => {
                    let mut is_neq = Number::null_value();
                    is_neq.borrow_mut().set_context(self.context.clone());

                    for (a, b) in zip(&self.elements, &value.elements) {
                        let result = a.borrow_mut().perform_operation("!=", b.clone());

                        if result.is_err() {
                            return Err(result.err().unwrap());
                        }

                        is_neq = result.ok().unwrap();
                    }

                    Ok(is_neq)
                }
                ">" => {
                    let is_gt =
                        Number::from((self.elements.len() > value.elements.len()) as u8 as f64);
                    is_gt.borrow_mut().set_context(self.context.clone());

                    Ok(is_gt)
                }
                "<" => {
                    let is_lt =
                        Number::from((self.elements.len() < value.elements.len()) as u8 as f64);
                    is_lt.borrow_mut().set_context(self.context.clone());

                    Ok(is_lt)
                }
                ">=" => {
                    let is_gte =
                        Number::from((self.elements.len() >= value.elements.len()) as u8 as f64);
                    is_gte.borrow_mut().set_context(self.context.clone());

                    Ok(is_gte)
                }
                "<=" => {
                    let is_lte =
                        Number::from((self.elements.len() <= value.elements.len()) as u8 as f64);
                    is_lte.borrow_mut().set_context(self.context.clone());

                    Ok(is_lte)
                }
                "and" => {
                    let is_and = Number::from(
                        (!self.elements.is_empty() && !value.elements.is_empty()) as u8 as f64,
                    );
                    is_and.borrow_mut().set_context(self.context.clone());

                    Ok(is_and)
                }
                "or" => {
                    let is_or = Number::from(
                        (!self.elements.is_empty() || !value.elements.is_empty()) as u8 as f64,
                    );
                    is_or.borrow_mut().set_context(self.context.clone());

                    Ok(is_or)
                }
                _ => Err(self.illegal_operation(Some(other.clone()))),
            },
            Value::NumberValue(ref value) => match operator {
                "^" => {
                    if value.value < -1.0 {
                        return Err(StandardError::new(
                            "cannot access a negative index",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            Some(
                                "use an index greater than or equal to 0 or use -1 to reverse the list",
                            ),
                        ));
                    }

                    if value.value == -1.0 {
                        return Ok(self.reverse());
                    }

                    if (value.value as usize) >= self.elements.len() {
                        return Err(StandardError::new(
                            "index is out of bounds",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            None,
                        ));
                    }

                    Ok(self.retrieve(value.value as usize))
                }
                "-" => {
                    if value.value < 0.0 {
                        return Err(StandardError::new(
                            "cannot access a negative index",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            Some("use an index greater than or equal to 0"),
                        ));
                    }

                    if (value.value as usize) >= self.elements.len() {
                        return Err(StandardError::new(
                            "index is out of bounds",
                            value.pos_start.clone().unwrap(),
                            value.pos_end.clone().unwrap(),
                            None,
                        ));
                    }

                    Ok(self.remove(value.value as usize))
                }
                _ => Err(self.illegal_operation(Some(other.clone()))),
            },
            _ => Err(self.illegal_operation(Some(other.clone()))),
        }
    }

    pub fn illegal_operation(&self, other: Option<Rc<RefCell<Value>>>) -> StandardError {
        StandardError::new(
            "operation not supported by type",
            self.pos_start.as_ref().unwrap().clone(),
            if other.is_some() {
                other.unwrap().borrow().position_end().unwrap()
            } else {
                self.pos_end.as_ref().unwrap().clone()
            },
            None,
        )
    }

    pub fn push(&mut self, item: Rc<RefCell<Value>>) -> Rc<RefCell<Value>> {
        self.elements.push(item);

        Number::null_value()
    }

    pub fn append(&mut self, other: &mut Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
        self.elements.append(other);

        Number::null_value()
    }

    pub fn remove(&mut self, index: usize) -> Rc<RefCell<Value>> {
        self.elements.remove(index).clone()
    }

    pub fn retrieve(&self, index: usize) -> Rc<RefCell<Value>> {
        self.elements[index].clone()
    }

    pub fn reverse(&mut self) -> Rc<RefCell<Value>> {
        self.elements.reverse();

        Number::null_value()
    }

    pub fn as_string(&self) -> String {
        let output = self
            .elements
            .iter()
            .map(|item| item.borrow().as_string())
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{output}]").to_string()
    }
}
