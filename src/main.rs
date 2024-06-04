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
        // If there is more then 1 param add a space under the description
        if parameters_node
            .named_children(&mut parameters_node.walk())
            .count()
            > 0
        {
            js_doc.add_space();
        }

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
    let start_byte = node.start_byte();
    let mut last_byte = start_byte;

    for child in node.children(&mut inner_cursor) {
        let child_start_byte = child.start_byte();
        updated_code.push_str(&source_code[last_byte..child_start_byte]);

        if child.kind() == "class_body" {
            process_class_body(source_code, &child, updated_code);
        } else {
            updated_code.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
        }

        last_byte = child.end_byte();
    }
    updated_code.push_str(&source_code[last_byte..node.end_byte()]);
}

fn process_class_body(source_code: &str, node: &Node, updated_code: &mut String) {
    let mut body_cursor = node.walk();
    let start_byte = node.start_byte();
    let mut last_byte = start_byte;

    for child in node.children(&mut body_cursor) {
        let child_start_byte = child.start_byte();
        updated_code.push_str(&source_code[last_byte..child_start_byte]);

        if child.kind() == "method_definition" {
            process_functions(source_code, &child, updated_code);
        } else if child.kind() == "class_declaration" {
            process_class_declaration(source_code, &child, updated_code);
        } else {
            updated_code.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
        }

        last_byte = child.end_byte();
    }
    updated_code.push_str(&source_code[last_byte..node.end_byte()]);
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
            return child
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .to_string()
                .trim()
                .to_string();
        } else if child.kind() == "function_declaration" {
            let mut export_cursor = child.walk();
            for export_child in child.children(&mut export_cursor) {
                if export_child.kind() == "identifier" {
                    return export_child
                        .utf8_text(source_code.as_bytes())
                        .unwrap()
                        .trim()
                        .to_string();
                }
            }
        }
    }
    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let source_code = r#"
            function testNoExport(param1: string, param2?: bool) {

            }
            
            export function testExport() {
            
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
            
            /**
             * testExport
             */
            export function testExport() {
            
            }
        "#;

        let updated_code = process(source_code);
        // println!("{}", updated_code);
        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_class() {
        let source_code = r#"
            class A {
                testNoExport(param1: string, param2?: bool) {
                    // TODO
                }

                public aa() {
                    // TODO
                }

                private b() {
                    // TODO
                }
            }
        "#;

        let expected_output = r#"
            class A {
                /**
                 * testNoExport
                 *
                 * @param {string} param1 - 
                 * @param {bool} [param2] - 
                 */
                testNoExport(param1: string, param2?: bool) {
                    // TODO
                }

                /**
                 * aa
                 */
                public aa() {
                    // TODO
                }

                /**
                 * b
                 */
                private b() {
                    // TODO
                }
            }
        "#;

        let updated_code = process(source_code);
        // println!("a {}", updated_code);
        assert_eq!(updated_code, expected_output);
    }
}
