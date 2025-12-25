#[derive(Debug, PartialEq)]
enum Token {
    Const(u8),
    Var(char),
    Operator(char),
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Constant(u8),
    Variable(char),
    BinOp(char, Box<Expr>, Box<Expr>),
}

fn tokenize(input: &str) -> Vec<Token> { 
    input
        .chars()
        .filter_map(|ch| match ch {
            ch if ch.is_whitespace() => None,
            ch if ch.is_digit(10) => Some(Token::Const(ch.to_digit(10).unwrap() as u8)),
            'a'..'z' | 'A'..'Z' => Some(Token::Var(ch)),
            '+' | '-' | '*' | '/' => Some(Token::Operator(ch)),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            _ => panic!("Unknown symbol: {}", ch)
        })
        .collect::<Vec<Token>>()
}

fn parse(input: &str) -> Expr {
    let tokens = tokenize(input);
    let (expr, rest) = parse_expr(&tokens, 0);

    if !rest.is_empty() {
        panic!("Extra tokens at the end!");
    }

    expr
}

fn parse_expr(tokens: &[Token], min_prec: u8) -> (Expr, &[Token]) {
    let (mut expr, mut rest) = parse_primary(tokens);

    while let Some(token) = rest.first() { 
        if let Token::Operator(op) = token {
            let prec = precedence(*op);

            if prec < min_prec + 1 {
                break;
            }

            rest = &rest[1..];
            let (right, new_rest) = parse_expr(rest, prec);
            expr = Expr::BinOp(*op, Box::new(expr), Box::new(right));
            rest = new_rest;

        } else {
            break;
        }
    }

    (expr, rest)
}

fn parse_primary(tokens: &[Token]) -> (Expr, &[Token]) {
    match tokens.first() {
        Some(Token::Var(c)) => (Expr::Variable(*c), &tokens[1..]),
        Some(Token::Const(n)) => (Expr::Constant(*n), &tokens[1..]),

        Some(Token::LParen) => {
            let (expr, rest) = parse_expr(&tokens[1..], 0);
            match rest.first() {
                Some(Token::RParen) => (expr, &rest[1..]),
                _ => panic!("Ожидалась закрывающая скобка"),
            }
        }

        _ => panic!("Ожидалось число, переменная или ("),
    }
}

fn precedence(op: char) -> u8 {
    match op {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0
    }
}

fn main() {
    // 1. Простые атомы
    assert_eq!(parse("x"), Expr::Variable('x'));
    assert_eq!(parse("4"), Expr::Constant(4));
    assert_eq!(parse("  3  "), Expr::Constant(3));

    // 2. Простые бинарные операции
    assert_eq!(
        parse("a + b"),
        Expr::BinOp('+', Box::new(Expr::Variable('a')), Box::new(Expr::Variable('b')))
    );

    assert_eq!(
        parse("x*y"),
        Expr::BinOp('*', Box::new(Expr::Variable('x')), Box::new(Expr::Variable('y')))
    );

    // 3. Приоритеты
    assert_eq!(
        parse("a + b * c"),
        Expr::BinOp(
            '+',
            Box::new(Expr::Variable('a')),
            Box::new(Expr::BinOp('*', Box::new(Expr::Variable('b')), Box::new(Expr::Variable('c'))))
        )
    );

    assert_eq!(
        parse("a + b - c * d"),
        Expr::BinOp(
            '-',
            Box::new(Expr::BinOp('+', Box::new(Expr::Variable('a')), Box::new(Expr::Variable('b')))),
            Box::new(Expr::BinOp('*', Box::new(Expr::Variable('c')), Box::new(Expr::Variable('d'))))
        )
    );

    assert_eq!(
        parse("a * b + c"),
        Expr::BinOp(
            '+',
            Box::new(Expr::BinOp('*', Box::new(Expr::Variable('a')), Box::new(Expr::Variable('b')))),
            Box::new(Expr::Variable('c'))
        )
    );

    // 4. Скобки меняют приоритет
    assert_eq!(
        parse("(a + b) * c"),
        Expr::BinOp(
            '*',
            Box::new(Expr::BinOp('+', Box::new(Expr::Variable('a')), Box::new(Expr::Variable('b')))),
            Box::new(Expr::Variable('c'))
        )
    );

    // 5. Сложные цепочки
    assert_eq!(
        parse("a + b + c * d * e - f / g"),
        Expr::BinOp(
            '-',
            Box::new(Expr::BinOp(
                '+',
                Box::new(Expr::BinOp(
                    '+',
                    Box::new(Expr::Variable('a')),
                    Box::new(Expr::Variable('b'))
                )),
                Box::new(Expr::BinOp(
                    '*',
                    Box::new(Expr::BinOp('*', Box::new(Expr::Variable('c')), Box::new(Expr::Variable('d')))),
                    Box::new(Expr::Variable('e'))
                ))
            )),
            Box::new(Expr::BinOp(
                '/',
                Box::new(Expr::Variable('f')),
                Box::new(Expr::Variable('g'))
            ))
        )
    );

    // 6. Пробелы везде — не должны влиять
    assert_eq!(parse("  a  +  (  x  *  3  )  "), parse("a+(x*3)"));

    // === Ошибки: должны падать в panic! ===

    macro_rules! should_panic {
        ($input:expr) => {{
            let result = std::panic::catch_unwind(|| parse($input));
            assert!(result.is_err(), "Ожидалась паника на входе: {}", $input);
        }};
    }

    should_panic!("");                    // пустая строка
    should_panic!("a +");                 // незавершённое выражение
    should_panic!("a + b +");             // тоже
    should_panic!("(a + b");              // незакрытая скобка
    should_panic!("a + b)");              // лишняя скобка
    should_panic!("a + * b");             // два оператора подряд
    should_panic!("++a");                 // оператор в начале
    should_panic!("a b");                 // два идентификатора подряд
    should_panic!("123abc");              // число + буква без оператора
    should_panic!("a! + b");              // неизвестный символ
    should_panic!("@");                   // любой мусор
    should_panic!("   ");                 // только пробелы

    println!("Все тесты прошли!");
}
