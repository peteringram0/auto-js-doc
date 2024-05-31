use regex::Regex;
use tree_sitter::{Node, Parser};
use tree_sitter_typescript::language_typescript;

fn main() {
    let mut parser = Parser::new();
    parser
        // .set_language(&tree_sitter_rust::language())
        .set_language(&language_typescript())
        .expect("Error loading Rust grammar");

    let source_code = "
    export function test() {
        
    }

    function testNoExport() {
        
    }
    ";

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    let updated_code = walk(&root_node, source_code);

    println!("origional code: {}", source_code);
    println!("updated_code: {}", updated_code);
}

// Returns indentation of a node as a string of the indentation characters
fn get_indentation(source_code: &str, node: &Node) -> String {
    let start_byte = node.start_byte();
    let line_start_byte = source_code[..start_byte].rfind('\n').map_or(0, |n| n + 1);
    let indentation = source_code[line_start_byte..start_byte]
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();
    indentation
}

fn get_function_name(line: &str) -> String {
    let re = Regex::new(r"(?:export\s+)?function\s+(\w+)").unwrap();
    let captures = re.captures(line).unwrap();
    captures.get(1).unwrap().as_str().to_owned()
}

fn walk(node: &Node, source_code: &str) -> String {
    let mut cursor = node.walk();
    let mut updated_code = String::new();
    let mut last_byte = 0;

    for child in node.children(&mut cursor) {
        let child_start_byte = child.start_byte();
        let child_end_byte = child.end_byte();

        // Append the text from the end of the last child to the start of the current child
        updated_code.push_str(&source_code[last_byte..child_start_byte]);

        if child.kind() == "export_statement" || child.kind() == "function_declaration" {
            // current lint
            let function_name_line = child.utf8_text(source_code.as_bytes()).unwrap().to_owned();

            // grab function name from string
            let function_name = get_function_name(&function_name_line);

            // get indentation of line
            let indentation = get_indentation(source_code, &child);

            // formatted comment to prepend
            let comment_str = format!("// {}\n", function_name);
            updated_code.push_str(&comment_str);

            // add the node
            let node = child.utf8_text(source_code.as_bytes()).unwrap();
            updated_code.push_str(&format!("{}{}", indentation, node));
        } else {
            // Append the text of the current child node
            updated_code.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
        }

        // Update last_byte to the end of the current child
        last_byte = child_end_byte;
    }

    // Append any remaining text after the last child
    updated_code.push_str(&source_code[last_byte..]);

    updated_code
}
