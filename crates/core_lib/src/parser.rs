use std::collections::HashMap;

use crate::ast::*;
use crate::errors::{ErrorCodes, ParseError};
use regex::Regex;

// --- HELPER FUNCTIONS ---
pub fn is_toplevel_keyword(line: &str) -> bool {
    let s_up = line.to_uppercase();
    s_up == "USING" || s_up.starts_with("REQUEST ") || s_up == "RESPONSE"
}

pub fn trim_quotes(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

pub fn is_identifier_like(s: &str) -> bool {
    // Match plain identifiers (name, field_name, etc.) or handlebars-style variables ({varname})
    let re = Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*|\{[A-Za-z_][A-Za-z0-9_]*\})$").unwrap();
    re.is_match(s)
}

fn find_top_level_logical(tokens: &[String]) -> Option<(usize, String)> {
    for (i, t) in tokens.iter().enumerate() {
        let tu = t.to_uppercase();
        if tu == "AND" || tu == "OR" {
            return Some((i, tu));
        }
    }

    None
}

// --- PARSER FUNCTIONS ---
pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    let mut using_block: Option<UsingBlock> = None;
    let mut request_blocks: Vec<RequestBlock> = Vec::new();
    let mut response_blocks: Vec<ResponseBlock> = Vec::new();

    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    // Clean Comments
    for l in lines.iter_mut() {
        if l.is_empty() {
            continue;
        }
        if let Some(index) = l.find("#") {
            l.truncate(index);
        }
        *l = l.trim().to_string();
    }

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        // --- Parse USING Block ---
        if line.eq_ignore_ascii_case("USING") {
            i += 1;
            let mut vars = Vec::new();
            while i < lines.len() {
                let l = lines[i].trim();
                if l.is_empty() {
                    i += 1;
                    break;
                }

                if is_toplevel_keyword(l) {
                    break;
                }

                if let Some(colon) = l.find(':') {
                    let name = l[..colon].trim().to_string();
                    let value = trim_quotes(l[colon + 1..].trim());
                    vars.push(VarDeclaration { name, value });
                } else {
                    return Err(ParseError::Syntax {
                        line: i + 1,
                        column: 1,
                        message: ErrorCodes::UnexpectedToken(l.to_string()),
                    });
                }

                i += 1;
            }

            using_block = Some(UsingBlock {
                var_declarations: vars,
            });
            continue;

        // --- Parse REQUEST Block ---
        } else if line.to_uppercase().starts_with("REQUEST ") {
            // Extract Request Name
            let request_name = line["REQUEST ".len()..].trim().to_string();
            i += 1;

            let mut method = HttpMethods::Get;
            let mut url = String::new();
            let mut headers: Vec<Header> = Vec::new();
            let mut cache = CacheDuration::None;

            while i < lines.len() {
                let l = lines[i].trim();
                if l.is_empty() {
                    i += 1;
                    break;
                }

                if is_toplevel_keyword(l) {
                    break;
                }

                // Parse Method
                let upper = l.to_uppercase();
                if upper.starts_with("GET ")
                    || upper.starts_with("DELETE ")
                    || upper.starts_with("PATCH ")
                    || upper.starts_with("PUT ")
                    || upper.starts_with("POST ")
                {
                    let parts: Vec<&str> = l.split_whitespace().collect();
                    method = match parts[0].to_uppercase().as_str() {
                        "GET" => HttpMethods::Get,
                        "POST" => HttpMethods::Post,
                        "PUT" => HttpMethods::Put,
                        "DELETE" => HttpMethods::Delete,
                        "PATCH" => HttpMethods::Patch,
                        _ => HttpMethods::Get,
                    };
                    url = l[parts[0].len()..].trim().to_string();
                }
                // Parse Headers
                else if upper.starts_with("HEADER ") {
                    i += 1;
                    while i < lines.len() {
                        let hline = lines[i].trim();
                        if hline.is_empty() || is_toplevel_keyword(hline) {
                            break;
                        }

                        if let Some(colon) = hline.find(':') {
                            let key = hline[..colon].trim().to_string();
                            let value = trim_quotes(hline[colon + 1..].trim());
                            headers.push(Header { key, value });
                        } else {
                            break;
                        }

                        i += 1;
                    }
                    continue;
                }
                // Parse Cache
                else if upper.starts_with("CACHE ") {
                    if let Some(num_s) = l.split_whitespace().nth(1) {
                        if let Ok(n) = num_s.parse::<u64>() {
                            cache = CacheDuration::DurationInSeconds(n);
                        }
                    }
                }

                i += 1;
            }

            request_blocks.push(RequestBlock {
                name: request_name,
                method,
                url,
                headers,
                cache,
            });
            continue;
        }
        // --- Parse RESPONSE Block ---
        else if line.eq_ignore_ascii_case("RESPONSE") {
            i += 1;

            let mut from_clause: Option<FromClause> = None;
            let mut where_clause: Option<Expression> = None;
            let mut select_clause: Option<SelectClause> = None;
            let mut limit: Option<u32> = None;

            while i < lines.len() {
                let l = lines[i].trim();
                // Skip empty lines and comments within RESPONSE block
                if l.is_empty() || l.starts_with('#') {
                    i += 1;
                    continue;
                }

                if is_toplevel_keyword(l) {
                    break;
                }

                if l.to_uppercase().starts_with("FROM ") {
                    let rest = l["FROM ".len()..].trim();

                    //body
                    if rest.starts_with("body.") {
                        let path: Vec<String> = rest["body.".len()..]
                            .split('.')
                            .map(|s| s.to_string())
                            .collect();

                        from_clause = Some(FromClause {
                            from_type: FromType::Body,
                            path,
                        });
                    } else if rest.starts_with("response.") {
                        let path: Vec<String> = rest["response.".len()..]
                            .split('.')
                            .map(|s| s.to_string())
                            .collect();

                        from_clause = Some(FromClause {
                            from_type: FromType::Response,
                            path,
                        });
                    } else {
                        // Fallback to body
                        let path: Vec<String> = rest.split('.').map(|s| s.to_string()).collect();
                        from_clause = Some(FromClause {
                            from_type: FromType::Body,
                            path,
                        });
                    }
                } else if l.to_uppercase().starts_with("WHERE ") {
                    let expr_text = l["WHERE".len()..].trim();
                    where_clause = Some(parse_expression(expr_text)?);
                } else if l.to_uppercase().starts_with("ORDER BY ") {
                    // TODO: Implement ORDER BY parsing
                } else if l.to_uppercase().starts_with("SELECT ") {
                    // Check if it's object select
                    if l.contains('{') {
                        let mut select_text = String::new();
                        if l.contains('}') {
                            // Inline Select
                            if let Some(start) = l.find('{') {
                                if let Some(end) = l.find('}') {
                                    select_text = l[start + 1..end].trim().to_string();
                                }
                            }
                        } else {
                            // Different lines
                            if let Some(start) = l.find('{') {
                                select_text.push_str(&l[start + 1..]);
                            }
                            i += 1;
                            while i < lines.len() {
                                let s = lines[i].trim();
                                if let Some(end) = s.find('}') {
                                    select_text.push_str(" ");
                                    select_text.push_str(&s[..end]);
                                    break;
                                } else {
                                    select_text.push_str(" ");
                                    select_text.push_str(s);
                                }
                                i += 1;
                            }
                        }

                        // Parse select fields
                        let parts: Vec<&str> = select_text
                            .split(',')
                            .map(|p| p.trim())
                            .filter(|p| !p.is_empty())
                            .collect();
                        let mut fields: Vec<SelectField> = Vec::new();
                        for p in parts {
                            // Preicate
                            if let Some(colon) = p.find(':') {
                                let alias = p[..colon].trim().to_string();
                                let expr_str = p[colon + 1..].trim();
                                let expression = parse_expression(expr_str)?;
                                fields.push(SelectField {
                                    alias,
                                    expression: Some(expression),
                                });
                            } else {
                                fields.push(SelectField {
                                    alias: p.to_string(),
                                    expression: None,
                                });
                            }
                        }
                        select_clause = Some(SelectClause::Objects(fields));
                    } else {
                        // Simple Fields
                        let fields_text = l["SELECT ".len()..].trim();
                        let fields: Vec<String> = fields_text
                            .split(',')
                            .map(|f| f.trim().to_string())
                            .collect();
                        select_clause = Some(SelectClause::Fields(fields));
                    }
                } else if l.to_uppercase().starts_with("LIMIT ") {
                    if let Some(num_s) = l.split_whitespace().nth(1) {
                        if let Ok(n) = num_s.parse::<u32>() {
                            limit = Some(n);
                        } else {
                            return Err(ParseError::Syntax {
                                line: i + 1,
                                column: 1,
                                message: ErrorCodes::InvalidLimitValue(num_s.to_string()),
                            });
                        }
                    }
                }
                i += 1;
            }

            let q = QueryBlock {
                select_clause: select_clause.unwrap_or(SelectClause::Fields(Vec::new())),
                from_clause: from_clause.unwrap_or(FromClause {
                    from_type: FromType::Body,
                    path: Vec::new(),
                }),
                where_clause,
                limit,
            };

            response_blocks.push(ResponseBlock { query: q });

            continue;
        } else {
            //TODO: Handle unexpected lines
            i += 1;
        }
    }

    let mut program = Program {
        using_block,
        request_blocks,
        response_blocks,
    };

    unroll_handlers(&mut program);
    Ok(program)
}

/// --- VARIABLE UNROLLING ---
pub fn unroll_handlers(program: &mut Program) {
    let mut vars: HashMap<String, String> = HashMap::new();
    if let Some(using) = &program.using_block {
        for var in &using.var_declarations {
            vars.insert(var.name.clone(), var.value.clone());
        }
    }

    // Helper Regex for Handlebars variable replacement
    let hb_regex = Regex::new(r"\{([A-Za-z_][A-Za-z0-9_]*)\}").unwrap();
    let replace_vars = |text: &str| -> String {
        let result = hb_regex.replace_all(text, |caps: &regex::Captures| {
            let var_name = &caps[1];
            if let Some(value) = vars.get(var_name) {
                value.clone()
            } else {
                caps[0].to_string()
            }
        });
        result.to_string()
    };

    // Replace in Request Blocks
    for req in program.request_blocks.iter_mut() {
        req.url = replace_vars(&req.url);
        for header in req.headers.iter_mut() {
            header.value = replace_vars(&header.value);
        }
    }

    // Replace in Response Blocks
    for resp in program.response_blocks.iter_mut() {
        // where clause
        if let Some(expr) = &mut resp.query.where_clause {
            resolve_expr(expr, &vars);
        }

        match &mut resp.query.select_clause {
            SelectClause::Fields(fields) => {
                for f in fields.iter_mut() {
                    *f = replace_vars(f);
                }
            }
            SelectClause::Objects(obj_fields) => {
                for of in obj_fields.iter_mut() {
                    if let Some(expr) = &mut of.expression {
                        resolve_expr(expr, &vars);
                    }
                }
            }
        }
    }
}

pub fn resolve_expr(expr: &mut Expression, vars: &HashMap<String, String>) {
    match expr {
        Expression::LiteralExpr(lit) => {
            if let Literal::StringLiteral(s) = lit {
                let re = Regex::new(r"^\{([A-Za-z_][A-Za-z0-9_]*)\}$").unwrap();
                if let Some(caps) = re.captures(s) {
                    let name = caps.get(1).unwrap().as_str();
                    if let Some(val) = vars.get(name) {
                        // attempt to parse number or boolean
                        if let Ok(n) = val.parse::<f64>() {
                            *lit = Literal::NumberLiteral(n);
                        } else if val.eq_ignore_ascii_case("true") {
                            *lit = Literal::BooleanLiteral(true);
                        } else if val.eq_ignore_ascii_case("false") {
                            *lit = Literal::BooleanLiteral(false);
                        } else {
                            *lit = Literal::StringLiteral(val.clone());
                        }
                    }
                } else {
                    let hb_re = Regex::new(r"\{([A-Za-z_][A-Za-z0-9_]*)\}").unwrap();
                    let replaced = hb_re.replace_all(s, |caps: &regex::Captures| {
                        let name = &caps[1];
                        vars.get(name)
                            .cloned()
                            .unwrap_or_else(|| format!("{{{}}}", name))
                    });
                    *lit = Literal::StringLiteral(replaced.into_owned());
                }
            }
        }
        Expression::FieldPathExpr(_) => {}
        Expression::BinaryOpExpr { left, right, .. } => {
            resolve_expr(left, vars);
            resolve_expr(right, vars);
        }
    }
}

// --- EXPRESSION PARSING ---
fn tokenize_expr(s: &str) -> Vec<String> {
    let s = s.trim();
    let re = Regex::new(r"(>=|<=|!=|==|=~|>|<|=|\band\b|\bAND\b|\bor\b|\bOR\b)").unwrap();
    let mut out = Vec::new();
    let mut last = 0usize;

    for m in re.find_iter(s) {
        if m.start() > last {
            out.push(s[last..m.start()].trim().to_string());
        }

        out.push(s[m.start()..m.end()].trim().to_string());
        last = m.end();
    }

    if last < s.len() {
        out.push(s[last..].trim().to_string());
    }

    //Split by whitespace
    out.into_iter()
        .flat_map(|chunk| {
            if chunk.contains(' ') {
                chunk.split_whitespace().map(|s| s.to_string()).collect()
            } else {
                vec![chunk]
            }
        })
        .collect()
}

pub fn parse_expression(s: &str) -> Result<Expression, ParseError> {
    let tokens: Vec<String> = tokenize_expr(s);

    if tokens.is_empty() {
        return Err(ParseError::Syntax {
            line: 0,
            column: 0,
            message: ErrorCodes::UnexpectedToken(s.to_string()),
        });
    }

    if let Some((pos, op)) = find_top_level_logical(&tokens) {
        let left = tokens[..pos].join(" ");
        let right = tokens[pos + 1..].join(" ");
        let left_expr = parse_expression(&left)?;
        let right_expr = parse_expression(&right)?;

        let binop = match op.as_str() {
            "AND" => BinaryOp::And,
            "OR" => BinaryOp::Or,
            _ => {
                return Err(ParseError::Syntax {
                    line: 0,
                    column: 0,
                    message: ErrorCodes::UnexpectedToken(op),
                });
            }
        };

        return Ok(Expression::BinaryOpExpr {
            left: Box::new(left_expr),
            op: binop,
            right: Box::new(right_expr),
        });
    }

    let mut op_pos: Option<(usize, &str)> = None;
    for (idx, t) in tokens.iter().enumerate() {
        let up = t.to_uppercase();
        match up.as_str() {
            ">=" | "<=" | "!=" | "==" | "=~" | ">" | "<" | "=" => {
                op_pos = Some((idx, t.as_str()));
                break;
            }
            _ => {}
        }
    }

    if let Some((pos, op_token)) = op_pos {
        let left = tokens[..pos].join(" ");
        let right = tokens[pos + 1..].join(" ");
        let left_expr = parse_term(&left)?;
        let right_expr = parse_term(&right)?;
        let binop = match op_token {
            ">=" => BinaryOp::Gte,
            "<=" => BinaryOp::Lte,
            ">" => BinaryOp::Gt,
            "<" => BinaryOp::Lt,
            "!=" => BinaryOp::Neq,
            "=~" => BinaryOp::RegexMatch,
            "==" | "=" => BinaryOp::Eq,
            _ => BinaryOp::Eq,
        };
        return Ok(Expression::BinaryOpExpr {
            left: Box::new(left_expr),
            op: binop,
            right: Box::new(right_expr),
        });
    }
    parse_term(&s)
}

fn parse_term(s: &str) -> Result<Expression, ParseError> {
    let t = s.trim();
    if t.is_empty() {
        return Err(ParseError::Syntax {
            line: 0,
            column: 0,
            message: ErrorCodes::UnexpectedToken(s.to_string()),
        });
    }

    if is_identifier_like(t) || t.contains(".") {
        let path_parts: Vec<String> = t.split('.').map(|p| p.trim().to_string()).collect();
        return Ok(Expression::FieldPathExpr(FieldPath { path: path_parts }));
    }

    if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
        let content = trim_quotes(t);
        return Ok(Expression::LiteralExpr(Literal::StringLiteral(content)));
    }

    // boolean
    if t.eq_ignore_ascii_case("true") {
        return Ok(Expression::LiteralExpr(Literal::BooleanLiteral(true)));
    }

    if t.eq_ignore_ascii_case("false") {
        return Ok(Expression::LiteralExpr(Literal::BooleanLiteral(false)));
    }

    // number
    if let Ok(n) = t.parse::<f64>() {
        return Ok(Expression::LiteralExpr(Literal::NumberLiteral(n)));
    }

    let re = Regex::new(r"^\{([A-Za-z_][A-Za-z0-9_]*)\}$").unwrap();
    if let Some(caps) = re.captures(t) {
        let name = caps.get(1).unwrap().as_str().to_string();
        // represent as StringLiteral for now; will be resolved & converted later
        return Ok(Expression::LiteralExpr(Literal::StringLiteral(format!(
            "{{{}}}",
            name
        ))));
    }

    // fallback treat as string literal
    Ok(Expression::LiteralExpr(Literal::StringLiteral(
        t.to_string(),
    )))
}
