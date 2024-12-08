use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Expression, Statement},
    environment::Environment,
    object::{EvalError, Object},
    parser::Parser,
};

#[derive(Debug)]
pub struct Evaluator<'a> {
    parser: Parser<'a>,
    env: Rc<RefCell<Environment>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(input: &'a str) -> Self {
        let parser = Parser::new(input);
        let env = Rc::new(RefCell::new(Environment::default()));

        Evaluator { parser, env }
    }

    pub fn eval_program(&mut self) -> Result<Vec<Object>, EvalError> {
        let program = self.parser.parse_program()?;
        let mut objects: Vec<Object> = vec![];

        for statement in program.0 {
            let obj = self.eval_statement(statement)?;
            objects.push(obj);
        }

        Ok(objects)
    }

    fn eval_statement(&mut self, statement: Statement) -> Result<Object, EvalError> {
        match statement {
            Statement::AssignStatement { name, value } => {
                let obj = self.eval_expression(value);
                self.env.borrow_mut().set(name, obj);
                Ok(Object::NullValue)
            }

            Statement::ExpressionStatement(expr) => Ok(self.eval_expression(expr)),

            Statement::BlockStatement(statements) => {
                let inner_env = self.create_enclosed_env();
                let outer_env = std::mem::replace(&mut self.env, inner_env);

                // save last evaluated object
                let mut obj = Object::NullValue;

                for statement in statements {
                    // evaluate all other types of statements
                    obj = self.eval_statement(statement)?;
                }

                // go back to the outer environment
                self.env = outer_env;

                // return the last evaluated object
                Ok(obj)
            }

            Statement::PrintStatement(expr) => {
                let obj = self.eval_expression(expr);
                println!("{obj}");
                Ok(Object::NullValue)
            }
        }
    }

    fn eval_expression(&mut self, expr: Expression) -> Object {
        match expr {
            Expression::IntegerLiteral(lit) => Object::IntegerValue(lit),
            Expression::Identifier(name) => self.env.borrow().get(&name),
        }
    }

    /// Creates a new environment linked to the outer environment
    fn create_enclosed_env(&mut self) -> Rc<RefCell<Environment>> {
        let inner_env = Environment {
            outer: Some(self.env.clone()),
            ..Default::default()
        };
        Rc::new(RefCell::new(inner_env))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_integer_literal() {
        let input = "5";
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[0];
        assert_eq!(result, &Object::IntegerValue(5));
    }

    #[test]
    fn eval_block_statement() {
        let input = r#"
            a = 2

            scope {
                b = 3
                b
            }

            a
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[2];
        assert_eq!(result, &Object::IntegerValue(2));
    }

    #[test]
    fn eval_assign_statement() {
        let input = r#"
            a = 2
            a
        "#;
        let mut evaluator = Evaluator::new(input);
        let result = &evaluator.eval_program().unwrap()[1];
        assert_eq!(result, &Object::IntegerValue(2));
    }
}
