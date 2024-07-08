use crate::lexer::lexer::Lexer;
use crate::lexer::token::token::{Token, TokenType};
use crate::parser::ast::ast::{Identifier, LetStatement, Program, Statement};
use crate::parser::parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexer,
        parser::ast::ast::{
            Expression, ExpressionStatement, InfixExpression, IntegerLiteral, Node,
            PrefixExpression,
        },
    };

    #[test]
    fn test_let_and_return_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        return 5;
        return 15;
        return 55;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 6);

        let tests = vec![
            ("x", 5),
            ("y", 10),
            ("foobar", 838383),
            ("", 5),
            ("", 15),
            ("", 55),
        ];

        for (i, (expected_ident, expected_value)) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            test_let_and_return_statement(stmt, expected_ident, *expected_value);
        }
    }

    fn test_let_and_return_statement(stmt: &Statement, name: &str, value: i64) {
        match stmt {
            Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.token.kind, TokenType::Let);
                assert_eq!(let_stmt.identifier.name, name);
                assert_eq!(let_stmt.identifier.get_lexeme(), name);
                assert_eq!(let_stmt.value.get_lexeme(), value.to_string());
            }
            Statement::Return(return_stmt) => {
                assert_eq!(return_stmt.token.kind, TokenType::Return);
                assert_eq!(return_stmt.value.get_lexeme(), value.to_string());
            }
            _ => panic!("stmt is not a LetStatement. Got={:?}", stmt),
        }
    }

    fn check_parser_errors(parser: &Parser) {
        let errors = &parser.errors;
        if errors.is_empty() {
            return;
        }

        println!("parser has {} errors", errors.len());
        for error in errors {
            println!("parser error: {}", error);
        }
        panic!("parser errors encountered");
    }

    #[test]
    fn test_identifiers_and_integer_literals_statements() {
        let input = "
        foobar;
        x;
        y;
        5;
        54;
        90;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 6);

        let expected_values = vec!["foobar", "x", "y", "5", "54", "90"];

        for (i, expected_value) in expected_values.iter().enumerate() {
            let stmt = &program.statements[i];
            test_identifier_or_integer_literal(stmt, expected_value);
        }
    }

    fn test_identifier_or_integer_literal(stmt: &Statement, expected_value: &str) {
        match stmt {
            Statement::Expression(expression_stmt) => match &expression_stmt.expression {
                Expression::Identifier(identifier_exp) => {
                    assert_eq!(identifier_exp.get_lexeme(), expected_value);
                }
                Expression::Integer(integer_literal) => {
                    assert_eq!(integer_literal.get_lexeme(), expected_value);
                }
                _ => panic!("stmt is not an Identifier or an Integer. Got={:?}", stmt),
            },
            _ => panic!("stmt is not an ExpressionStatement. Got={:?}", stmt),
        }
    }
    #[test]
    fn test_prefix_expressions() {
        let prefix_tests = vec![("-5", "-", 5), ("!5", "!", 5)];

        for (input, operator, value) in prefix_tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1, "Expected 1 statement");

            let stmt = &program.statements[0];
            match stmt {
                Statement::Expression(ExpressionStatement { expression, .. }) => match expression {
                    Expression::Prefix(PrefixExpression {
                        operator: op,
                        right,
                        ..
                    }) => {
                        assert_eq!(
                            op, operator,
                            "Expected operator '{}', got '{}'",
                            operator, op
                        );
                        match right.as_ref() {
                            Expression::Integer(IntegerLiteral {
                                value: int_value, ..
                            }) => {
                                assert_eq!(
                                    *int_value, value,
                                    "Expected value '{}', got '{}'",
                                    value, int_value
                                );
                            }
                            _ => panic!("Expected integer literal, got {:?}", right),
                        }
                    }
                    _ => panic!("Expected prefix expression, got {:?}", expression),
                },
                _ => panic!("Expected expression statement, got {:?}", stmt),
            }
        }
    }

    #[test]
    fn test_infix_expressions() {
        let infix_tests = vec![("6-5", "-", 6, 5), ("10*5", "*", 10, 5)];

        for (input, operator, left_value, right_value) in infix_tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1, "Expected 1 statement");

            let stmt = &program.statements[0];
            match stmt {
                Statement::Expression(ExpressionStatement { expression, .. }) => match expression {
                    Expression::Infix(InfixExpression {
                        left,
                        operator: op,
                        right,
                        ..
                    }) => {
                        match left.as_ref() {
                            Expression::Integer(IntegerLiteral {
                                value: int_value, ..
                            }) => {
                                assert_eq!(
                                    *int_value, left_value,
                                    "Expected value '{}', got '{}'",
                                    left_value, int_value
                                );
                            }
                            _ => panic!("Expected integer literal, got {:?}", right),
                        }
                        assert_eq!(
                            op, operator,
                            "Expected operator '{}', got '{}'",
                            operator, op
                        );
                        match right.as_ref() {
                            Expression::Integer(IntegerLiteral {
                                value: int_value, ..
                            }) => {
                                assert_eq!(
                                    *int_value, right_value,
                                    "Expected value '{}', got '{}'",
                                    right_value, int_value
                                );
                            }
                            _ => panic!("Expected integer literal, got {:?}", right),
                        }
                    }
                    _ => panic!("Expected prefix expression, got {:?}", expression),
                },
                _ => panic!("Expected expression statement, got {:?}", stmt),
            }
        }
    }
}
