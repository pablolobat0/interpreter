#[cfg(test)]
mod test {

    use crate::{
        common::lexer::lexer::Lexer,
        vm::{
            chunk::{Chunk, OpCode, Value},
            compiler::Compiler,
            vm::{InterpretResult, VirtualMachine},
        },
    };

    fn check_compiler_errors(compiler: &Compiler) {
        let errors = &compiler.errors;
        if errors.is_empty() {
            return;
        }

        println!("compiler has {} errors", errors.len());
        for error in errors {
            println!("compiler error: {}", error);
        }
        panic!("compiler errors encountered");
    }

    fn test_number(input: &str, result: f64) {
        let mut lexer = Lexer::new(input);
        let mut compiler = Compiler::new(&mut lexer);
        compiler.compile_one_statement();

        check_compiler_errors(&compiler);

        let mut vm = VirtualMachine::new(&mut compiler);

        assert_eq!(
            vm.interpret(),
            InterpretResult::Ok,
            "VM should run without errors"
        );

        assert_eq!(vm.stack.get(0), Some(&Value::Number(result)));
    }

    fn test_bool(input: &str, result: bool) {
        let mut lexer = Lexer::new(input);
        let mut compiler = Compiler::new(&mut lexer);

        compiler.compile_one_statement();

        check_compiler_errors(&compiler);

        let mut vm = VirtualMachine::new(&mut compiler);

        assert_eq!(
            vm.interpret(),
            InterpretResult::Ok,
            "VM should run without errors"
        );

        assert_eq!(vm.stack.get(0), Some(&Value::Boolean(result)));
    }

    fn test_string(input: &str, result: String) {
        let mut lexer = Lexer::new(input);
        let mut compiler = Compiler::new(&mut lexer);

        compiler.compile_one_statement();

        check_compiler_errors(&compiler);

        let mut vm = VirtualMachine::new(&mut compiler);

        assert_eq!(
            vm.interpret(),
            InterpretResult::Ok,
            "VM should run without errors"
        );
        println!("{}", input);

        assert_eq!(vm.stack.get(0), Some(&Value::String(result)));
    }

    #[test]
    fn test_constant() {
        test_number("1", 1.0);
    }

    #[test]
    fn test_boolean() {
        let tests = [("true", true), ("false", false)];

        for (input, result) in tests {
            test_bool(input, result);
        }
    }

    #[test]
    fn test_null() {
        let mut lexer = Lexer::new("null");
        let mut compiler = Compiler::new(&mut lexer);

        assert!(
            compiler.compile_one_statement(),
            "Compiler should compile without errors"
        );

        let mut vm = VirtualMachine::new(&mut compiler);

        assert_eq!(
            vm.interpret(),
            InterpretResult::Ok,
            "VM should run without errors"
        );

        assert_eq!(vm.stack.get(0), Some(&Value::Null));
    }

    #[test]
    fn test_negate() {
        test_number("-1.2", -1.2);
    }

    #[test]
    fn test_add() {
        test_number("10+5", 15.0);
    }

    #[test]
    fn test_subtract() {
        test_number("10-5", 5.0);
    }

    #[test]
    fn test_multiply() {
        test_number("10*5", 50.0);
    }

    #[test]
    fn test_divide() {
        test_number("10/5", 2.0);
    }

    #[test]
    fn test_division_by_zero() {
        let input = "10 / 0";
        let mut lexer = Lexer::new(input);
        let mut compiler = Compiler::new(&mut lexer);

        assert!(
            compiler.compile_one_statement(),
            "Compiler should compile without errors"
        );

        let mut vm = VirtualMachine::new(&mut compiler);
        assert_eq!(
            vm.interpret(),
            InterpretResult::RuntimeError,
            "VM should return a runtime error for division by zero"
        );
    }

    #[test]
    fn test_not() {
        let tests = [
            ("!true", false),
            ("!false", true),
            ("!null", true),
            ("!1", false),
        ];

        for (input, result) in tests {
            test_bool(input, result);
        }
    }

    #[test]
    fn test_boolean_infix() {
        let tests = [
            ("true == true", true),
            ("false == true", false),
            ("\"hola\" == \"hola\"", true),
            ("\"hola\" == \"mundo\"", false),
            ("1 == 1", true),
            ("1 == 2", false),
            ("1 != 1", false),
            ("1 != 2", true),
            ("\"hola\" != \"hola\"", false),
            ("\"hola\" != \"mundo\"", true),
            ("true != true", false),
            ("true != false", true),
            ("1 > 0", true),
            ("1 > 1", false),
            ("1 >= 1", true),
            ("1 >= 2", false),
            ("1 < 1", false),
            ("1 < 2", true),
            ("1 <= 1", true),
            ("1 <= 0", false),
        ];

        for (input, result) in tests {
            test_bool(input, result);
        }
    }

    #[test]
    fn test_string_literal() {
        test_string("\"hola\"", "hola".to_string());
    }

    #[test]
    fn test_string_concatenation() {
        test_string("\"hola\" + \" mundo\"", "hola mundo".to_string());
    }

    #[test]
    fn test_define_global() {
        let input = "let a = 1";

        let mut lexer = Lexer::new(input);
        let mut compiler = Compiler::new(&mut lexer);

        compiler.compile();

        check_compiler_errors(&compiler);

        let mut vm = VirtualMachine::new(&mut compiler);

        assert_eq!(
            vm.interpret(),
            InterpretResult::Ok,
            "VM should run without errors"
        );

        assert_eq!(vm.globals.get("a"), Some(&Value::Number(1.0)));
    }

    #[test]
    fn test_get_global() {
        let input = "let a = 1\nlet b = a + 3";

        let mut lexer = Lexer::new(input);
        let mut compiler = Compiler::new(&mut lexer);

        compiler.compile();

        check_compiler_errors(&compiler);

        let mut vm = VirtualMachine::new(&mut compiler);

        assert_eq!(
            vm.interpret(),
            InterpretResult::Ok,
            "VM should run without errors"
        );

        assert_eq!(vm.globals.get("b"), Some(&Value::Number(4.0)));
    }

    #[test]
    fn test_set_global() {
        let input = "let a = 1\na = 3";

        let mut lexer = Lexer::new(input);
        let mut compiler = Compiler::new(&mut lexer);

        compiler.compile();

        check_compiler_errors(&compiler);

        let mut vm = VirtualMachine::new(&mut compiler);

        assert_eq!(
            vm.interpret(),
            InterpretResult::Ok,
            "VM should run without errors"
        );

        assert_eq!(vm.globals.get("a"), Some(&Value::Number(3.0)));
    }
}
