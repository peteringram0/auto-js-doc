#![allow(dead_code)]

mod structs;

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
fn get_params(source_code: &str, child: &Node, js_doc: &mut JsDoc) {
    if let Some(parameters_node) = child.child_by_field_name("parameters") {
        for param in parameters_node.named_children(&mut parameters_node.walk()) {
            let mut param_name: Option<String> = None;
            let mut param_type: Option<String> = None;
            let param_required = param.kind() == "required_parameter";

            for child in param.named_children(&mut param.walk()) {
                if child.kind() == "identifier" {
                    param_name = Some(child.utf8_text(source_code.as_bytes()).unwrap().to_owned());
                }
                if child.kind() == "type_annotation" {
                    if let Some(type_node) = child.named_child(0) {
                        param_type = Some(
                            type_node
                                .utf8_text(source_code.as_bytes())
                                .unwrap()
                                .to_owned(),
                        );
                    }
                }

                if let (Some(param_name), Some(param_type)) =
                    (param_name.as_ref(), param_type.as_ref())
                {
                    js_doc.add_param(param_name, param_type, !param_required, "");
                }
            }
        }
    }
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
            process_functions(source_code, &child, &mut updated_code);
        } else if child.kind() == "class_declaration" {
            process_class_declaration(source_code, &child, &mut updated_code);
        } else {
            updated_code.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
        }

        // Update last_byte to the end of the current child
        last_byte = child_end_byte;
    }

    // Append any remaining text after the last child
    updated_code.push_str(&source_code[last_byte..]);

    updated_code
}

fn process_class_declaration(source_code: &str, node: &Node, updated_code: &mut String) {
    let mut inner_cursor = node.walk();
    for child in node.children(&mut inner_cursor) {
        if child.kind() == "class_body" {
            process_class_body(source_code, &child, updated_code);
        } else {
            updated_code.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
        }
    }
}

fn process_class_body(source_code: &str, node: &Node, updated_code: &mut String) {
    let mut body_cursor = node.walk();
    for child in node.children(&mut body_cursor) {
        if child.kind() == "method_definition" {
            process_functions(source_code, &child, updated_code);
        } else if child.kind() == "class_declaration" {
            process_class_declaration(source_code, &child, updated_code);
        } else {
            updated_code.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
        }
    }
}

fn process_functions(source_code: &str, node: &Node, updated_code: &mut String) {
    let indentation = get_indentation(source_code, node);
    let mut js_doc = JsDoc::new(&indentation);

    js_doc.add_description(&get_function_name_from_node(source_code, node));

    get_params(source_code, node, &mut js_doc);

    updated_code.push_str(&format!("{}\n", js_doc.build())); // add in the JsDoc

    // add the node
    let node = node.utf8_text(source_code.as_bytes()).unwrap();
    updated_code.push_str(&format!("{}{}", indentation, node));
}

fn get_function_name_from_node(source_code: &str, node: &Node) -> String {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "property_identifier" || child.kind() == "identifier" {
            return child.utf8_text(source_code.as_bytes()).unwrap().to_string();
        }
    }
    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        let source_code = r#"
            function testNoExport(param1: string, param2?: bool) {

            }
        "#;

        let expected_output = r#"
            /**
             * testNoExport
             *
             * @param {string} param1 - 
             * @param {bool} [param2] - 
             */
            function testNoExport(param1: string, param2?: bool) {

            }
        "#;

        let updated_code = process(source_code);
        // println!("{}", updated_code)
        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_class() {
        // TODO
        let source_code = r#"
            class A {
                testNoExport(param1: string, param2?: bool) {
                    // TODO
                }

                aa() {
                    // TODO
                }
            }
        "#;

        // let expected_output = r#"
        //     /**
        //      * testNoExport
        //      *
        //      * @param {string} param1 -
        //      * @param {bool} [param2] -
        //      */
        //     function testNoExport(param1: string, param2?: bool) {

        //     }
        // "#;

        let updated_code = process(source_code);
        println!("updated: {}", updated_code)
        // assert_eq!(updated_code, expected_output);
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
             * @param {string} param1 - 
             */
            function testNoExport(param1: string) {

            }
        "#;

        let updated_code = process(source_code);
        assert_eq!(updated_code, expected_output);
    }
}
