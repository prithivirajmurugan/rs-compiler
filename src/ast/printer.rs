use crate::ast::*;
use crate::text::span::TextSpan;
use termion::color::{self, Fg, Reset};

use super::visitor::ASTVisitor;

pub struct ASTPrinter {
    indent: usize,
    pub result: String,
}

impl ASTPrinter {
    const NUMBER_COLOR: color::Cyan = color::Cyan;
    const TEXT_COLOR: color::LightWhite = color::LightWhite;
    const KEYWORD_COLOR: color::Magenta = color::Magenta;
    const VARIABLE_COLOR: color::Green = color::Green;
    const BOOLEAN_COLOR: color::Yellow = color::Yellow;
    const TYPE_COLOR: color::LightBlue = color::LightBlue;

    fn add_whitespace(&mut self) {
        self.result.push_str(" ")
    }
    fn add_newline(&mut self) {
        self.result.push_str(
            "
        ",
        );
    }
    fn add_keyword(&mut self, keyword: &str) {
        self.result
            .push_str(&format!("{}{}", Self::KEYWORD_COLOR.fg_str(), keyword,));
    }
    fn add_text(&mut self, text: &str) {
        self.result
            .push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), text));
    }
    fn add_variable(&mut self, variable: &str) {
        self.result
            .push_str(&format!("{}{}", Self::VARIABLE_COLOR.fg_str(), variable,));
    }
    fn add_padding(&mut self) {
        for _ in 0..self.indent {
            self.result.push_str(" ");
        }
    }
    fn add_boolean(&mut self, boolean: bool) {
        self.result
            .push_str(&format!("{}{}", Self::BOOLEAN_COLOR.fg_str(), boolean,));
    }
    fn add_type(&mut self, type_: &str) {
        self.result
            .push_str(&format!("{}{}", Self::TYPE_COLOR.fg_str(), type_,));
    }
    fn add_type_annotation(&mut self, type_annotation: &StaticTypeAnnotation) {
        self.add_text(":");
        self.add_whitespace();
        self.add_type(&type_annotation.type_name.span.literal);
    }

    pub fn new() -> Self {
        Self {
            indent: 0,
            result: String::new(),
        }
    }
}

impl ASTVisitor for ASTPrinter {
    fn visit_func_expr(&mut self, ast: &mut Ast, func_expr: &FuncExpr, expr_id: ExprId) {
        self.add_keyword("func");
        self.add_whitespace();
        let decl = &func_expr.decl;
        let are_parameters_empty = decl.parameters.is_empty();
        if !are_parameters_empty {
            self.add_text("(");
        } else {
            self.add_whitespace();
        }
        for (i, parameter) in decl.parameters.iter().enumerate() {
            if i != 0 {
                self.add_text(",");
                self.add_whitespace();
            }
            self.add_text(&parameter.identifier.span.literal);
            self.add_type_annotation(&parameter.type_aannotation);
        }
        if !are_parameters_empty {
            self.add_text(")");
            self.add_whitespace();
        }
        self.visit_expression(ast, decl.body);
    }
    fn visit_return_statement(&mut self, ast: &mut Ast, return_statement: &ReturnStmt) {
        self.add_keyword("return");
        if let Some(expression) = &return_statement.return_value {
            self.add_whitespace();
            self.visit_expression(ast, *expression);
        }
    }
    fn visit_while_statment(&mut self, ast: &mut Ast, while_statement: &WhileStmt) {
        self.add_keyword("while");
        self.add_whitespace();
        self.visit_expression(ast, while_statement.condition);
        self.add_whitespace();
        self.visit_expression(ast, while_statement.body);
    }
    fn visit_block_expr(&mut self, ast: &mut Ast, block_statement: &BlockExpr, expr: &Expr) {
        self.add_text("{");
        self.add_newline();
        self.indent += 1;
        for statement in &block_statement.stmts {
            self.visit_statement(ast, *statement);
        }
        self.indent -= 1;
        self.add_padding();
        self.add_text("}");
    }
    fn visit_if_expression(&mut self, ast: &mut Ast, if_statement: &IfExpr, expr: &Expr) {
        self.add_keyword("if");
        self.add_whitespace();
        self.visit_expression(ast, if_statement.condition);
        self.add_whitespace();
        self.visit_expression(ast, if_statement.then_branch);
        if let Some(else_branch) = &if_statement.else_branch {
            self.add_whitespace();
            self.add_keyword("else");
            self.add_whitespace();
            self.visit_expression(ast, else_branch.expr);
        }
    }
    fn visit_statement(&mut self, ast: &mut Ast, statement: StmtId) {
        self.add_padding();
        self.do_visit_statement(ast, statement);
        self.result.push_str(&format!("{}\n", Fg(Reset),))
    }
    fn visit_rec_expression(&mut self, ast: &mut Ast, expr: &RecExpr, expr_id: ExprId) {
        self.add_keyword("rec");
    }
    fn visit_call_expression(&mut self, ast: &mut Ast, call_expression: &CallExpr, expr: &Expr) {
        self.visit_expression(ast, call_expression.callee);
        self.add_text("(");
        for (i, argument) in call_expression.arguements.iter().enumerate() {
            if i != 0 {
                self.add_text(",");
                self.add_whitespace();
            }
            self.visit_expression(ast, *argument);
        }
        self.add_text(")");
    }
    fn visit_assignment_expression(
        &mut self,
        ast: &mut Ast,
        assignment_expression: &AssignExpr,
        expr: &Expr,
    ) {
        self.add_variable(assignment_expression.identifier.span.literal.as_str());
        self.add_whitespace();
        self.add_text("=");
        self.add_whitespace();
        self.visit_expression(ast, assignment_expression.expression);
    }
    fn visit_variable_expression(
        &mut self,
        ast: &mut Ast,
        variable_expression: &VarExpr,
        expr: &Expr,
    ) {
        self.result.push_str(&format!(
            "{}{}",
            Self::VARIABLE_COLOR.fg_str(),
            variable_expression.identifier.span.literal,
        ));
    }
    fn visit_number_expression(&mut self, ast: &mut Ast, number: &NumberExpr, expr: &Expr) {
        self.result
            .push_str(&format!("{}{}", Self::NUMBER_COLOR.fg_str(), number.number,));
    }
    fn visit_boolean_expression(&mut self, ast: &mut Ast, boolean: &BoolExpr, expr: &Expr) {
        self.add_boolean(boolean.value);
    }
    fn visit_error(&mut self, ast: &mut Ast, span: &TextSpan) {
        self.result
            .push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), span.literal,));
    }
    fn visit_unary_expression(&mut self, ast: &mut Ast, unary_expression: &UnaryExpr, expr: &Expr) {
        self.result.push_str(&format!(
            "{}{}",
            Self::TEXT_COLOR.fg_str(),
            unary_expression.operator.token.span.literal,
        ));
        self.visit_expression(ast, unary_expression.operand);
    }

    fn visit_binary_expression(
        &mut self,
        ast: &mut Ast,
        binary_expression: &BinaryExpr,
        expr: &Expr,
    ) {
        self.visit_expression(ast, binary_expression.left);
        self.add_whitespace();
        self.result.push_str(&format!(
            "{}{}",
            Self::TEXT_COLOR.fg_str(),
            binary_expression.operator.token.span.literal,
        ));
        self.add_whitespace();
        self.visit_expression(ast, binary_expression.right);
    }
    fn visit_parenthesized_expression(
        &mut self,
        ast: &mut Ast,
        parenthesized_expression: &ParenthesizedExpr,
        expr: &Expr,
    ) {
        self.result
            .push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), "(",));
        self.visit_expression(ast, parenthesized_expression.expression);
        self.result
            .push_str(&format!("{}{}", Self::TEXT_COLOR.fg_str(), ")",));
    }
}
