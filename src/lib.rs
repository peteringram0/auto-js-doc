mod e2e_test;
mod structs;

use std::io::{self, Read, Write};
use structs::JsDoc;
use tree_sitter::{Node, Parser};
use tree_sitter_typescript::language_typescript;

#[derive(Debug, Default)]
struct FunctionInfo {
    function_name: String,
    return_type: Option<String>,
}
impl FunctionInfo {
    fn new(function_name: String, return_type: Option<String>) -> FunctionInfo {
        FunctionInfo {
            function_name,
            return_type,
        }
    }
}

pub fn main() {
    // Create a handle to stdin
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Create a string to hold the entire input
    let mut input = String::new();

    // Read the entire input into the string
    let output = match handle.read_to_string(&mut input) {
        Ok(_) => process(&input),
        Err(_) => "".to_owned(),
    };

    // Create a handle to stdout
    let stdout = io::stdout();
    let mut handle_out = stdout.lock();

    // Write the processed input to stdout
    if let Err(e) = writeln!(handle_out, "{}", output) {
        eprintln!("Error writing to stdout: {}", e);
    }

    io::stdout().flush().ok();
}

pub fn process(source_code: &str) -> String {
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
    // if child.kind() == "export_statement" {
    //     println!(
    //         "here {:?}",
    //         child
    //             .child(0)
    //             .unwrap()
    //             .next_named_sibling()
    //             .unwrap()
    //             .child_by_field_name("parameters")
    //     ); // TODO reports as none for exported !!
    // }

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
            let mut param_default: Option<String> = None;
            let param_required = param.kind() == "required_parameter";

            for child in param.named_children(&mut param.walk()) {
                // println!("here {}", child.kind());
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

                // Get any default value assigned
                if child.kind() == "string" {
                    param_default = Some(
                        child
                            .utf8_text(source_code.as_bytes())
                            .unwrap()
                            .to_owned()
                            .to_owned()
                            .trim_matches('"')
                            .to_owned(),
                    );
                }
            }

            // println!(
            //     "name: {:?}, type: {:?}, default: {:?}",
            //     param_name, param_type, param_default
            // );

            if let (Some(param_name), param_type) = (param_name.as_ref(), param_type.clone()) {
                js_doc.add_param(
                    param_name,
                    param_type,
                    !param_required,
                    param_default.clone(),
                    "",
                );
            }
        }
    }
}

fn walk(node: &Node, source_code: &str) -> String {
    let mut cursor = node.walk();
    let mut updated_code = String::new();
    let mut last_byte = 0;

    let mut comment: Option<String> = None;

    for child in node.children(&mut cursor) {
        let child_start_byte = child.start_byte();
        let child_end_byte = child.end_byte();

        // Append the text from the end of the last child to the start of the current child
        // updated_code.push_str(&source_code[last_byte..child_start_byte]);
        // Get the text between the last child and the current child
        let text_between = &source_code[last_byte..child_start_byte];

        if child.kind() == "comment" {
            // updated_code.push('\n');
            comment = Some(parse_comment(
                child.utf8_text(source_code.as_bytes()).unwrap(),
            ));
        } else {
            updated_code.push_str(text_between);
            if child.kind() == "export_statement" || child.kind() == "function_declaration" {
                process_functions(source_code, &child, &mut updated_code, &comment);
            } else if child.kind() == "class_declaration" {
                process_class_declaration(source_code, &child, &mut updated_code);
            } else {
                updated_code.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
            }
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

    let mut comment: Option<String> = None;

    for child in node.children(&mut body_cursor) {
        let child_start_byte = child.start_byte();

        // println!(
        //     "child: {} line: ${:?}",
        //     child.kind(),
        //     child.utf8_text(source_code.as_bytes())
        // );

        // Get the text between the last child and the current child
        let text_between = &source_code[last_byte..child_start_byte];

        if child.kind() == "comment" {
            // Skip the comment but handle surrounding whitespace
            // if let Some(last_newline) = text_between.rfind('\n') {
            //     updated_code.push_str(&text_between[..=last_newline]);
            // }
            updated_code.push('\n');

            // comment = Some("my comment".to_owned());
            // comment = Some(child.utf8_text(source_code.as_bytes()).unwrap().to_owned());
            comment = Some(parse_comment(
                child.utf8_text(source_code.as_bytes()).unwrap(),
            ));

            // println!("text_between: {:?}", &text_between);
            last_byte = child.end_byte();
            continue;
        } else {
            updated_code.push_str(text_between);

            // println!("class body kind: ${:?}", child.kind());

            if child.kind() == "method_definition" {
                process_functions(source_code, &child, updated_code, &comment);
            } else if child.kind() == "class_declaration" {
                process_class_declaration(source_code, &child, updated_code);
            } else {
                updated_code.push_str(child.utf8_text(source_code.as_bytes()).unwrap());
            }
        }

        last_byte = child.end_byte();
    }
    updated_code.push_str(&source_code[last_byte..node.end_byte()]);
}

fn process_functions(
    source_code: &str,
    node: &Node,
    updated_code: &mut String,
    comment: &Option<String>,
) {
    let indentation = get_indentation(source_code, node);
    let mut js_doc = JsDoc::new(&indentation);

    let info = get_function_details_from_node(source_code, node);
    // println!("info: {:?}", info);

    match comment {
        Some(comment) => {
            // let comment = child.utf8_text(source_code.as_bytes())
            js_doc.add_description(comment);
        }
        None => {
            js_doc.add_description(&info.function_name);
        }
    }

    // println!("comment ... within function: {:?}", comment);

    if node.kind() == "export_statement" {
        let params = node.child(0).unwrap().next_named_sibling().unwrap();
        get_params(source_code, &params, &mut js_doc);
    } else {
        get_params(source_code, node, &mut js_doc);
    }

    if let Some(return_type) = info.return_type {
        js_doc.add_return(&return_type, "");
    }

    updated_code.push_str(&format!("{}\n", js_doc.build())); // add in the JsDoc

    // add the node
    let node = node.utf8_text(source_code.as_bytes()).unwrap();
    updated_code.push_str(&format!("{}{}", indentation, node));
}

fn get_function_details_from_node(source_code: &str, node: &Node) -> FunctionInfo {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        // println!("aa: {:?}", return_type);
        if child.kind() == "property_identifier" || child.kind() == "identifier" {
            let return_type = get_function_return_type_from_node(source_code, node);
            let name = child
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .to_string()
                .trim()
                .to_string();
            return FunctionInfo::new(name, return_type);
        } else if child.kind() == "function_declaration" {
            let return_type = get_function_return_type_from_node(source_code, &child);
            let mut export_cursor = child.walk();
            for export_child in child.children(&mut export_cursor) {
                if export_child.kind() == "identifier" {
                    let name = export_child
                        .utf8_text(source_code.as_bytes())
                        .unwrap()
                        .trim()
                        .to_string();
                    return FunctionInfo::new(name, return_type);
                }
            }
        }
    }
    FunctionInfo::new("unknown".to_owned(), None)
}

fn get_function_return_type_from_node(source_code: &str, node: &Node) -> Option<String> {
    let return_type = node.child_by_field_name("return_type");
    // println!("return t: {:?}", return_type);
    return_type
        .map(|t| t.utf8_text(source_code.as_bytes()).unwrap().to_string())
        .map(|s| s.trim_start_matches(':').trim().to_string())
}

/// Remove comment indication characters from the string
fn parse_comment(comment: &str) -> String {
    comment
        .replace("/n", "")
        .replace("/", "")
        .replace("*", "")
        .lines()
        .map(|line| line.trim())
        .filter(|line| {
            // println!("line: {:?}", line);
            line != &"" && !line.starts_with("@")
        })
        .map(|line| line.to_owned())
        .collect::<Vec<String>>()
        .join("")
}
