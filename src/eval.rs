use std::{any::Any, collections::HashMap, ops::Range, rc::Rc};

use peg::{error::ParseError, str::LineCol};

#[derive(Clone)]
pub enum Expr {
    Str(String),
    Num(f64),
    Call {
        ident: String,
        args: Vec<Expr>,
        span: Range<usize>,
    },
}

#[derive(Clone)]
pub struct Decl {
    pub args: Vec<String>,
    pub rhs: Expr,
    pub r#where: Vec<(String, Decl)>,
    pub span: Range<usize>,
}

pub struct BodkinCtx {
    decls: HashMap<String, Decl>,
    externs: HashMap<String, Fn>,
}

pub struct BodkinLocalCtx {
    decls: HashMap<String, Decl>,
    externs: HashMap<String, Fn>,
    r#where: HashMap<String, Decl>,
    args: HashMap<String, Dynamic>,
}

#[derive(Debug)]
pub enum BodkinError {
    SyntaxError(ParseError<LineCol>),
}

#[derive(Debug)]
pub enum BodkinEvalError {
    MissingDecl(String),
    TypeErr(Range<usize>),
    IncorrectNumberOfArgs(Range<usize>),
}

type Dynamic = Rc<dyn Any>;
type Fn = fn(Vec<Dynamic>, span: Range<usize>) -> Result<Dynamic, BodkinEvalError>;

impl Default for BodkinCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl BodkinCtx {
    pub fn new() -> Self {
        BodkinCtx {
            decls: HashMap::new(),
            externs: HashMap::new(),
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), BodkinError> {
        let parsed: Vec<(String, Decl)> =
            crate::parser::bodkin_parser::code(code).map_err(BodkinError::SyntaxError)?;

        for (ident, rhs) in parsed {
            self.decls.insert(ident, rhs);
        }

        Ok(())
    }

    pub fn add_extern(&mut self, ident: &str, fun: Fn) {
        self.externs.insert(ident.to_string(), fun);
    }

    fn eval_decl(
        &self,
        ident: &str,
        args: &[Dynamic],
        span: Option<Range<usize>>,
    ) -> Result<Dynamic, BodkinEvalError> {
        let decl = self
            .decls
            .get(ident)
            .ok_or(BodkinEvalError::MissingDecl(ident.to_string()))?;

        if decl.args.len() != args.len() {
            return Err(BodkinEvalError::IncorrectNumberOfArgs(
                if let Some(s) = span {
                    s
                } else {
                    decl.span.clone()
                },
            ));
        }

        let mut args_hash: HashMap<String, Dynamic> = HashMap::new();

        for (i, ident) in decl.args.iter().enumerate() {
            args_hash.insert(ident.clone(), args[i].clone());
        }

        let mut r#where: HashMap<String, Decl> = HashMap::new();

        for (ident, decl) in &decl.r#where {
            r#where.insert(ident.clone(), decl.clone());
        }

        BodkinLocalCtx {
            decls: self.decls.clone(),
            externs: self.externs.clone(),
            args: args_hash,
            r#where,
        }
        .eval_expr(&decl.rhs)
    }

    pub fn eval<T: 'static>(
        &self,
        ident: &str,
        args: &[Dynamic],
    ) -> Result<Rc<T>, BodkinEvalError> {
        self.eval_decl(ident, args, None)?
            .downcast::<T>()
            .or(Err(BodkinEvalError::TypeErr(0..0)))
    }
}

impl BodkinLocalCtx {
    fn eval_expr(&self, expr: &Expr) -> Result<Dynamic, BodkinEvalError> {
        match expr {
            Expr::Str(s) => {
                let d: Rc<dyn Any> = Rc::new(s.clone());
                Ok(d)
            }
            Expr::Num(n) => {
                let d: Rc<dyn Any> = Rc::new(*n);
                Ok(d)
            }
            Expr::Call { ident, args, span } => {
                if let Some(decl) = self.args.get(ident) {
                    return Ok(decl.clone());
                }

                let mut evaled_args = vec![];

                for arg in args {
                    evaled_args.push(self.eval_expr(arg)?)
                }

                if let Some(decl) = self.r#where.get(ident) {
                    self.eval_expr(&decl.rhs)
                } else if let Some(fun) = self.r#externs.get(ident) {
                    fun(evaled_args, span.clone())
                } else {
                    let mut evaled_args = vec![];

                    for arg in args {
                        evaled_args.push(self.eval_expr(arg)?)
                    }

                    BodkinCtx {
                        decls: self.decls.clone(),
                        externs: self.externs.clone(),
                    }
                    .eval_decl(ident, &evaled_args, Some(span.clone()))
                }
            }
        }
    }
}
