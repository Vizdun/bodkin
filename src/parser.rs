use crate::eval::{Decl, Expr};

peg::parser! {
  pub grammar bodkin_parser() for str {
    rule ident() -> String
      = i:['a'..='z' | 'A'..='Z']+ { i.iter().collect::<String>() }

    rule num() -> Expr
      = s:"-"? n:$(['0'..='9']+) d:("." n:$(['0'..='9']+) {n})? {? format!("{}{}.{}", match s { Some(_) => "-", None => "" }, n, d.unwrap_or("0")).parse().or(Err("f64")).map(Expr::Num) }

    rule str() -> Expr
      = "\"" s:("\\" s:[_] { vec![s] } / s:[c if !c.is_control() && c != '"']*) "\"" { Expr::Str(s.iter().collect()) }

    rule ident_expr() -> Expr
      = start:position!() i:ident() end:position!() { Expr::Call {
          ident: i,
          args: vec![],
          span: start..end
       } }

    rule call() -> (Expr)
      = start:position!() i:ident() whitespace_no_newline() a:((num()/str()/ident_expr()) ** " ") end:position!() { Expr::Call {
        ident: i,
        args: a,
        span: start..end
      } }

    rule whitespace() = quiet!{[' ' | '\n' | '\t']*}

    rule whitespace_no_newline() = quiet!{[' ' | '\t']*}

    rule decl_args() -> Vec<String>
      = a:(ident() ** " ") { a }

    rule decl() -> (String, Decl)
      = i:ident() whitespace_no_newline() start:position!() a:decl_args() end:position!() whitespace_no_newline() "=" whitespace_no_newline() e:(num()/str()/call()) { (i, Decl {
          args: a,
          rhs: e,
          r#where: vec![],
          span: start..end
      }) }

    rule decl_where() -> (String, Decl)
      = d:decl() w:(w:("\n" "  " w:decl() { w })* { w })? { (d.0, Decl{
            r#where: w.unwrap_or_default(),
          ..d.1
      }) }

    pub rule code() -> Vec<(String, Decl)>
      = whitespace() d:(d:decl_where() whitespace() { d })* { d }
  }
}
