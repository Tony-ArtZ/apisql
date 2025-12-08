use crate::ast::*;
use crate::errors::QueryError;
use serde_json::Value;

pub fn execute_query(query: &QueryBlock, body: &Value) -> Result<Value, QueryError> {
    let root = match query.from_clause.from_type {
        FromType::Body => body,
        FromType::Response => {
            return Err(QueryError::TypeError {
                message: "Response from_type not supported yet".to_string(),
            });
        }
    };

    let from_val = resolve_path(root, &query.from_clause.path)?;
    let rows = match from_val {
        Value::Array(arr) => arr.clone(),
        other => vec![other.clone()],
    };

    // Apply WHERE clause
    let filtered: Vec<Value> = rows
        .into_iter()
        .filter(|row| {
            if let Some(cond) = &query.where_clause {
                eval_bool_expr(cond, row).unwrap_or(false)
            } else {
                true
            }
        })
        .collect();

    // Apply SELECT clause
    let mapped: Vec<Value> = match &query.select_clause {
        SelectClause::Fields(fields) => filtered
            .into_iter()
            .map(|row| project_fields(&row, fields))
            .collect::<Result<Vec<_>, _>>()?,
        SelectClause::Objects(select_fields) => filtered
            .into_iter()
            .map(|row| project_object_fields(&row, select_fields))
            .collect::<Result<Vec<_>, _>>()?,
    };

    // Apply LIMIT clause
    let limited = if let Some(limit) = query.limit {
        mapped.into_iter().take(limit as usize).collect()
    } else {
        mapped
    };

    Ok(Value::Array(limited))
}

// Given a vector path, resolve it or throw
fn resolve_path<'a>(value: &'a Value, path: &[String]) -> Result<&'a Value, QueryError> {
    let mut current = value;
    for segment in path {
        match current {
            Value::Object(map) => {
                current = map.get(segment).ok_or(QueryError::MissingField {
                    field: segment.clone(),
                })?;
            }
            _ => {
                return Err(QueryError::TypeError {
                    message: format!("cannot access field `{}` on non-object", segment),
                });
            }
        }
    }
    Ok(current)
}

// Select specific fields from a JSON object
fn project_fields(row: &Value, fields: &[String]) -> Result<Value, QueryError> {
    let mut obj = serde_json::Map::new();
    for field in fields {
        let v = resolve_path(row, &[field.clone()])?;
        obj.insert(field.clone(), v.clone());
    }
    Ok(Value::Object(obj))
}

// Select object fields with optional expressions and stuff
fn project_object_fields(row: &Value, fields: &[SelectField]) -> Result<Value, QueryError> {
    let mut obj = serde_json::Map::new();
    for field in fields {
        let value = if let Some(expr) = &field.expression {
            // Compute the expression
            eval_expr(expr, row)?
        } else {
            // Just get the field from the row here
            resolve_path(row, &[field.alias.clone()])?.clone()
        };
        obj.insert(field.alias.clone(), value);
    }
    Ok(Value::Object(obj))
}

fn eval_bool_expr(expr: &Expression, row: &Value) -> Result<bool, QueryError> {
    let v = eval_expr(expr, row)?;
    match v {
        Value::Bool(b) => Ok(b),
        other => Err(QueryError::TypeError {
            message: format!("WHERE expression must be bool, got {:?}", other),
        }),
    }
}

fn eval_expr(expr: &Expression, row: &Value) -> Result<Value, QueryError> {
    match expr {
        Expression::LiteralExpr(Literal::NumberLiteral(n)) => {
            let num = serde_json::Number::from_f64(*n).ok_or(QueryError::TypeError {
                message: "Invalid number".to_string(),
            })?;
            Ok(Value::Number(num))
        }

        Expression::LiteralExpr(Literal::StringLiteral(s)) => Ok(Value::String(s.clone())),

        Expression::LiteralExpr(Literal::BooleanLiteral(b)) => Ok(Value::Bool(*b)),

        Expression::LiteralExpr(Literal::Null) => Ok(Value::Null),

        Expression::FieldPathExpr(fp) => resolve_path(row, &fp.path).cloned(),

        // Recursive binary operation evaluation
        Expression::BinaryOpExpr { left, op, right } => {
            let l = eval_expr(left, row)?;
            let r = eval_expr(right, row)?;
            eval_binary(&l, op, &r)
        }
    }
}

fn eval_binary(left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, QueryError> {
    match op {
        &BinaryOp::Eq => Ok(Value::Bool(left == right)),
        &BinaryOp::Neq => Ok(Value::Bool(left != right)),
        &BinaryOp::Gt | &BinaryOp::Gte | &BinaryOp::Lt | &BinaryOp::Lte => {
            let ln = left.as_f64().ok_or(QueryError::TypeError {
                message: "Left operand is not a number".to_string(),
            })?;
            let rn = right.as_f64().ok_or(QueryError::TypeError {
                message: "Right operand is not a number".to_string(),
            })?;

            let result = match op {
                &BinaryOp::Gt => ln > rn,
                &BinaryOp::Gte => ln >= rn,
                &BinaryOp::Lt => ln < rn,
                &BinaryOp::Lte => ln <= rn,
                _ => unreachable!(),
            };
            Ok(Value::Bool(result))
        }

        &BinaryOp::And | &BinaryOp::Or => {
            let lb = left.as_bool().ok_or(QueryError::TypeError {
                message: "Left operand is not a boolean".to_string(),
            })?;
            let rb = right.as_bool().ok_or(QueryError::TypeError {
                message: "Right operand is not a boolean".to_string(),
            })?;

            let result = match op {
                &BinaryOp::And => lb && rb,
                &BinaryOp::Or => lb || rb,
                _ => unreachable!(),
            };
            Ok(Value::Bool(result))
        }

        &BinaryOp::RegexMatch => {
            use regex::Regex;

            let text = left.as_str().ok_or(QueryError::TypeError {
                message: "Left operand for regex match must be a string".to_string(),
            })?;
            let pattern = right.as_str().ok_or(QueryError::TypeError {
                message: "Right operand for regex match must be a string pattern".to_string(),
            })?;

            let re = Regex::new(pattern).map_err(|e| QueryError::TypeError {
                message: format!("Invalid regex pattern: {}", e),
            })?;

            Ok(Value::Bool(re.is_match(text)))
        }
    }
}
