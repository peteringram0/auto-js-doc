pub struct JsDoc {
    formatted: String,
}

impl JsDoc {
    fn new() -> JsDoc {
        JsDoc {
            formatted: String::from("/**\n"),
        }
    }

    pub fn build(&mut self) -> String {
        self.formatted.push_str(" */");
        self.formatted.clone()
    }

    pub fn add_description(&mut self, description: &str) -> &mut JsDoc {
        self.formatted.push_str(&format!(" * {}\n", description));
        self.formatted.push_str(" *\n");
        self
    }

    // Method to add a parameter to the JsDoc
    pub fn add_param(&mut self, param: &str, param_type: &str, description: &str) -> &mut JsDoc {
        self.formatted.push_str(&format!(
            " * @param {{{}}} {} - {}\n",
            param_type, param, description
        ));
        self
    }

    // Method to add a return type to the JsDoc
    pub fn add_return(&mut self, return_type: &str, description: &str) -> &mut JsDoc {
        self.formatted
            .push_str(&format!(" * @return {{{}}} {}\n", return_type, description));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let mut builder = JsDoc::new();
        builder.add_description("add description");
        builder.add_param("foo", "string", "foo description");
        builder.add_param("bar", "string", "bar description");
        builder.add_return("string", "return of something");
        println!("{}", builder.build());
    }
}
