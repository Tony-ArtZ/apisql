use core_lib::*;

#[test]
fn test_basic_parsing() {
    let input = r#"
USING
  token: "secret123"
  baseUrl: "https://api.test.com"

REQUEST GetUsers
  GET {baseUrl}/users
  HEADER Authorization: Bearer {token}
  CACHE 60

RESPONSE
  FROM body.data
  WHERE age > 18
  SELECT {
    id,
    name
  }
  LIMIT 5
"#;

    let result = parse_program(input);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert!(program.using_block.is_some());
    assert_eq!(program.request_blocks.len(), 1);
    assert_eq!(program.response_blocks.len(), 1);
}

#[test]
fn test_where_clause() {
    let input = r#"
RESPONSE
  FROM body
  WHERE name == "John" AND age >= 25
"#;

    let result = parse_program(input);
    assert!(result.is_ok());

    let program = result.unwrap();
    assert_eq!(program.response_blocks.len(), 1);
    assert!(program.response_blocks[0].query.where_clause.is_some());
}

#[test]
fn test_empty_input() {
    let input = "";
    let result = parse_program(input);
    assert!(result.is_ok());
}

#[test]
fn test_comments() {
    let input = r#"
# This is a comment
USING
  token: "test" # inline comment

REQUEST GetData
  GET https://example.com # another comment
"#;

    let result = parse_program(input);
    assert!(result.is_ok());
}

#[test]
fn test_print_program_struct() {
    let input = r#"
USING
  token: "secret123"
  baseUrl: "https://api.test.com"

REQUEST GetUsers
  GET {baseUrl}/users
  HEADER Authorization: Bearer {token}
  CACHE 60

RESPONSE
  FROM body.data
  WHERE age > 18
  SELECT {
    id,
    name
  }
  LIMIT 5
"#;

    let result = parse_program(input);
    assert!(result.is_ok());

    let program = result.unwrap();
    println!("\n=== Program Structure ===");
    println!("Program struct fields:");
    println!("  - using_block: {:?}", program.using_block);
    println!(
        "  - request_blocks: {} blocks",
        program.request_blocks.len()
    );
    println!(
        "  - response_blocks: {} blocks",
        program.response_blocks.len()
    );

    if let Some(using) = &program.using_block {
        println!("\nUsing Block:");
        for var in &using.var_declarations {
            println!("  {} = {}", var.name, var.value);
        }
    }

    for (i, req) in program.request_blocks.iter().enumerate() {
        println!("\nRequest Block #{}:", i + 1);
        println!("  name: {}", req.name);
        println!("  method: {:?}", req.method);
        println!("  url: {}", req.url);
        println!("  headers: {} header(s)", req.headers.len());
        println!("  cache: {:?}", req.cache);
    }

    for (i, resp) in program.response_blocks.iter().enumerate() {
        println!("\nResponse Block #{}:", i + 1);
        println!("  query.from_clause: {:?}", resp.query.from_clause);
        println!("  query.where_clause: {:?}", resp.query.where_clause);
        println!("  query.select_clause: {:?}", resp.query.select_clause);
        println!("  query.limit: {:?}", resp.query.limit);
    }

    println!("\n=== Program struct is correctly defined ===\n");
}
