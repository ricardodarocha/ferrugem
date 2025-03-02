use crate::expr::{Expr, Expr::*, LiteralValue};
use crate::scanner::{Token, TokenType, TokenType::*};
use crate::stmt::Stmt;

// #[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    next_id: usize,
}

#[derive(Debug)]
enum FunctionKind {
    Function,
    Method,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            next_id: 0,
        }
    }

    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        id
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        let mut errs = vec![];

        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    errs.push(msg);
                    self.synchronize();
                }
            }
        }

        if errs.len() == 0 {
            Ok(stmts)
        } else {
            Err(errs.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(Var) {
            self.var_declaration()
        } else if self.match_token(Fun) {
            self.function(FunctionKind::Function)
        } else if self.match_token(Class) {
            self.class_declaration()
        } else {
            self.statement()
        }
    }

    fn class_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(Identifier, "Esperado o nome depois da palavra reservada 'classe' .")?;
        let superclass = if self.match_token(TokenType::Less) {
            self.consume(Identifier, "Esperada superclass depois do símbolo  '<'.")?;
            Some(Expr::Variable {
                id: self.get_id(),
                name: self.previous(),
            })
        } else {
            None
        };

        self.consume(LeftBrace, "Esperado '{' antes do corpo da classe.")?;

        let mut methods = vec![];
        while !self.check(RightBrace) && !self.is_at_end() {
            let method = self.function(FunctionKind::Method)?;
            methods.push(Box::new(method));
        }

        self.consume(RightBrace, "Esperado '}' depois do corpo da classe.")?;

        Ok(Stmt::Class {
            name,
            methods,
            superclass,
        })
    }

    fn function(&mut self, kind: FunctionKind) -> Result<Stmt, String> {
        let name = self.consume(Identifier, &format!("Esperado o tipo {kind:?} "))?;

        if self.match_token(Gets) {
            let cmd_body = self.consume(StringLit, "Esperado o corpo do comando")?; 
            self.consume(Semicolon, "Esperado ';' depois do corpo do comando")?;

            return Ok(Stmt::CmdFunction {
                name,
                cmd: cmd_body.lexeme,
            });
        }

        self.consume(LeftParen, &format!("Esperado '(' depois do tipo{kind:?} "))?;

        let mut parameters = vec![];
        if !self.check(RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!(
                        "Linha {location}: Estourou o limite de 255 argumentos"
                    ));
                }

                let param = self.consume(Identifier, "Esperado o nome do parâmetro")?;
                parameters.push(param);

                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(RightParen, "Esperado ')' depois dos parâmetros.")?;

        self.consume(LeftBrace, &format!("Esperado '{{' antes do tipo {kind:?}."))?;
        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Bloco inválido [134]"),
        };

        Ok(Stmt::Function {
            name,
            params: parameters,
            body,
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token = self.consume(Identifier, "Esperado o nome da variável")?;

        let initializer;
        if self.match_token(Equal) {
            initializer = self.expression()?;
        } else {
            initializer = Literal {
                id: self.get_id(),
                value: LiteralValue::Nil,
            };
        }

        self.consume(Semicolon, "Esperado ';' depois da declaração da variável")?;

        Ok(Stmt::Var {
            name: token,
            initializer,
        })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(Print) {
            self.print_statement()
        }
        else if self.match_token(Limpar) {
            self.limpa_tela()
        } else if self.match_token(LeftBrace) {
            self.block_statement()
        } else if self.match_token(If) {
            self.if_statement()
        } else if self.match_token(While) {
            self.while_statement()
        } else if self.match_token(For) {
            self.for_statement()
        } else if self.match_token(Return) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let keyword = self.previous();
        let value;
        if !self.check(Semicolon) {
            // NOT return;
            value = Some(self.expression()?);
        } else {
            value = None;
        }
        self.consume(Semicolon, "Esperado ';' depois do valor de retorno;")?;

        Ok(Stmt::ReturnStmt { keyword, value })
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        // for v
        //       ( SMTH ; SMTH ; SMTH )
        self.consume(LeftParen, "Esperado '(' depois 'for'.")?;

        // Consumes "SMTH ;"
        let initializer;
        if self.match_token(Semicolon) {
            initializer = None;
        } else if self.match_token(Var) {
            let var_decl = self.var_declaration()?;
            initializer = Some(var_decl);
        } else {
            let expr = self.expression_statement()?;
            initializer = Some(expr);
        }

        // Consumes "SMTH? ;"
        let condition;
        if !self.check(Semicolon) {
            let expr = self.expression()?;
            condition = Some(expr);
        } else {
            condition = None;
        }
        self.consume(Semicolon, "Esperado ';' depois da condição do laço.")?;

        let increment;
        if !self.check(RightParen) {
            let expr = self.expression()?;
            increment = Some(expr);
        } else {
            increment = None;
        }
        self.consume(RightParen, "Esperado ')' depois da cláusula para.")?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(Stmt::Expression { expression: incr }),
                ],
            };
        }

        let cond;
        match condition {
            None => {
                cond = Expr::Literal {
                    id: self.get_id(),
                    value: LiteralValue::True,
                }
            }
            Some(c) => cond = c,
        }
        body = Stmt::WhileStmt {
            condition: cond,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)],
            };
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Esperado '(' depois de 'enquanto'")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Esperado ')' depois da condição.")?;
        let body = self.statement()?;

        Ok(Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Esperado '(' depois do bloco 'se'")?;
        let predicate = self.expression()?;
        self.consume(RightParen, "Esperado ')' depois do predicado se")?;

        let then = Box::new(self.statement()?);
        let els = if self.match_token(Else) {
            let stm = self.statement()?;
            Some(Box::new(stm))
        } else {
            None
        };

        Ok(Stmt::IfStmt {
            predicate,
            then,
            els,
        })
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = vec![];

        while !self.check(RightBrace) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(Box::new(decl));
        }

        self.consume(RightBrace, "Esperado '}' depois do bloco")?;
        Ok(Stmt::Block { statements })
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        match self.consume(Semicolon, "Esperado ';' depois do valor.") {
            _=> () 
        };
        Ok(Stmt::Print { expression: value })
    }

    fn limpa_tela(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        match self.consume(Semicolon, "Esperado ';' depois do valor.") {
            _=> () 
        };
        Ok(Stmt::Limpar { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        match self.consume(Semicolon, "Esperado ';' depois da expressão."){
            _ => ()
        }
        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn function_expression(&mut self) -> Result<Expr, String> {
        let paren = self.consume(LeftParen, "Esperado '(' depois da função anônima")?;
        let mut parameters = vec![];
        if !self.check(RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!(
                        "Line {location}: Estourou o limite de 255 argumentos"
                    ));
                }

                let param = self.consume(Identifier, "Esperado o parâmetro nome")?;
                parameters.push(param);

                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(
            RightParen,
            "Esperado ')' depois dos parâmetros da função anônima",
        )?;

        self.consume(
            LeftBrace,
            "Esperado '{' depois da declaração da função anônima",
        )?;

        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Bloco inválido [360]"),
        };

        Ok(Expr::AnonFunction {
            id: self.get_id(),
            paren,
            arguments: parameters,
            body,
        })
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        // a = 2; NOT var a = 2;
        let expr = self.pipe()?; // a |> f = 2;

        if self.match_token(Equal) {
            let value = self.expression()?;

            match expr {
                Variable { id: _, name } => Ok(Assign {
                    id: self.get_id(),
                    name,
                    value: Box::from(value),
                }),
                Get {
                    id: _,
                    object,
                    name,
                } => Ok(Set {
                    id: self.get_id(),
                    object,
                    name,
                    value: Box::new(value),
                }),
                _ => Err("Destino inválido.".to_string()),
            }
        } else {
            Ok(expr)
        }
    }

    fn pipe(&mut self) -> Result<Expr, String> {
        // expr |> f
        // expr |> f1 |> f2
        // expr |> (f1 |> f2)
        // expr |> (f1 |> (f2 |> f3))
        // (expr |> f1) |> f2

        // expr |> fun (a) { return a + 1; }
        // expr |> a -> a + 1
        let mut expr = self.or()?;
        while self.match_token(Pipe) {
            let pipe = self.previous();
            let function = self.or()?;

            expr = Call {
                id: self.get_id(),
                callee: Box::new(function),
                paren: pipe,
                arguments: vec![expr],
            };
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        while self.match_token(Or) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.match_token(And) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let rhs = self.term()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[Minus, Plus]) {
            let op = self.previous();
            let rhs = self.factor()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_tokens(&[Slash, Star]) {
            let op = self.previous();
            let rhs = self.unary()?;
            expr = Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous();
            let rhs = self.unary()?;
            Ok(Unary {
                id: self.get_id(),
                operator: op,
                right: Box::from(rhs),
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(Dot) {
                let name = self.consume(Identifier, "Esperado token depois do ponto")?;
                expr = Get {
                    id: self.get_id(),
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = vec![];

        if !self.check(RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);
                if arguments.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!(
                        "Line {location}: Estourou o limite de 255 argumentos [571]"
                    ));
                }

                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        let paren = self.consume(RightParen, "Esperado ')' depois dos argumentos.")?;

        Ok(Call {
            id: self.get_id(),
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();
        let result;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Esperado ')'")?;
                result = Grouping {
                    id: self.get_id(),
                    expression: Box::from(expr),
                };
            }
            False | True | Nil | Number | StringLit => {
                self.advance();
                result = Literal {
                    id: self.get_id(),
                    value: LiteralValue::from_token(token),
                }
            }
            Identifier => {
                self.advance();
                result = Variable {
                    id: self.get_id(),
                    name: self.previous(),
                };
            }
            TokenType::This => {
                self.advance();
                result = Expr::This {
                    id: self.get_id(),
                    keyword: token,
                };
            }
            TokenType::Super => {
                // Should always occur with a method call
                self.advance();
                self.consume(TokenType::Dot, "Esperado '.' depois do comando 'super'.")?;
                let method =
                    self.consume(TokenType::Identifier, "Esperado o nome da classe principal.")?;
                result = Expr::Super {
                    id: self.get_id(),
                    keyword: token,
                    method,
                };
            }
            Fun => {
                self.advance();
                result = self.function_expression()?;
            }
            _ => return Err("Uma expressão era esperada".to_string()),
        }

        Ok(result)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            Err(format!("Line {}: {}", token.line_number, msg))
        }
    }

    fn check(&mut self, typ: TokenType) -> bool {
        self.peek().token_type == typ
    }

    fn match_token(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == typ {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        for typ in typs {
            if self.match_token(*typ) {
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == Eof
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{LiteralValue::*, Scanner};

    #[test]
    fn test_addition() {
        let one = Token {
            token_type: Number,
            lexeme: "1".to_string(),
            literal: Some(FValue(1.0)),
            line_number: 0,
        };
        let plus = Token {
            token_type: Plus,
            lexeme: "+".to_string(),
            literal: None,
            line_number: 0,
        };
        let two = Token {
            token_type: Number,
            lexeme: "2".to_string(),
            literal: Some(FValue(2.0)),
            line_number: 0,
        };
        let semicol = Token {
            token_type: Semicolon,
            lexeme: ";".to_string(),
            literal: None,
            line_number: 0,
        };
        let eof = Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: 0,
        };

        let tokens = vec![one, plus, two, semicol, eof];
        let mut parser = Parser::new(tokens);

        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_with_paren() {
        let source = "1 == (2 + 2);";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== 1 (group (+ 2 2)))");
    }
}
