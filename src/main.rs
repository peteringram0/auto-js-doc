#![allow(dead_code)]

mod structs;

use regex::Regex;
use structs::JsDoc;
use tree_sitter::{Node, Parser};
use tree_sitter_typescript::language_typescript;

fn main() {
    // TODO
}

// process
fn process(source_code: &str) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&language_typescript())
        .expect("Error loading Typescript grammar");

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    walk(&root_node, source_code)
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

// todo
fn get_params(source_code: &str, child: &Node) {
    if let Some(parameters_node) = child.child_by_field_name("parameters") {
        for param in parameters_node.named_children(&mut parameters_node.walk()) {
            let is_required = param.kind() == "required_parameter";
            println!("is required {}", is_required);

            for child in param.named_children(&mut param.walk()) {
                if child.kind() == "identifier" {
                    let param_name = child.utf8_text(source_code.as_bytes()).unwrap();
                    println!("Parameter name: {}", param_name);
                }
                if child.kind() == "type_annotation" {
                    if let Some(type_node) = child.named_child(0) {
                        let param_type = type_node.utf8_text(source_code.as_bytes()).unwrap();
                        println!("Parameter type: {}", param_type);
                    }
                }
            }
        }
    }
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
            let indentation = get_indentation(source_code, &child);
            let mut js_doc = JsDoc::new(&indentation);

            // current lint
            let function_name_line = child.utf8_text(source_code.as_bytes()).unwrap().to_owned(); // TODO get rid of this !

            get_params(source_code, &child); // TODO here

            js_doc.add_description(&get_function_name(&function_name_line));

            updated_code.push_str(&format!("{}\n", js_doc.build())); // add in the JsDoc

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        let source_code = r#"
            function testNoExport(param1: string) {

            }
        "#;

        let expected_output = r#"
            /**
             * testNoExport
             *
             */
            function testNoExport(param1: string) {

            }
        "#;

        let updated_code = process(source_code);

        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_no_comments() {
        let source_code = r#"
            export function test() {

            }

            function testNoExport(param1: string) {

            }
        "#;

        let expected_output = r#"
            /**
             * test
             *
             */
            export function test() {

            }

            /**
             * testNoExport
             *
             */
            function testNoExport(param1: string) {

            }
        "#;

        let updated_code = process(source_code);
        assert_eq!(updated_code, expected_output);
    }
}
