pub struct JsDoc {
    indentation: String,
    formatted: String,
}

impl JsDoc {
    pub fn new(indentation: &str) -> JsDoc {
        JsDoc {
            indentation: indentation.to_owned(),
            formatted: format!("{indentation}/**\n"),
        }
    }

    pub fn build(&mut self) -> String {
        self.formatted.push_str(&format!("{}*/", self.indentation));
        self.formatted.clone()
    }

    pub fn add_description(&mut self, description: &str) -> &mut JsDoc {
        self.formatted
            .push_str(&format!("{}* {}\n", self.indentation, description));
        self.formatted.push_str(&format!("{}*\n", self.indentation));
        self
    }

    // Method to add a parameter to the JsDoc
    pub fn add_param(&mut self, param: &str, param_type: &str, description: &str) -> &mut JsDoc {
        self.formatted.push_str(&format!(
            "{}* @param {{{}}} {} - {}\n",
            self.indentation, param_type, param, description
        ));
        self
    }

    // Method to add a return type to the JsDoc
    pub fn add_return(&mut self, return_type: &str, description: &str) -> &mut JsDoc {
        self.formatted.push_str(&format!(
            "{}* @return {{{}}} {}\n",
            self.indentation, return_type, description
        ));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let builder = JsDoc::new("")
            .add_description("add description")
            .add_param("foo", "string", "foo description")
            .add_param("bar", "string", "bar description")
            .add_return("string", "return of something")
            .build();

        let expected_output = r#"/**
* add description
*
* @param {string} foo - foo description
* @param {string} bar - bar description
* @return {string} return of something
*/"#;
        assert_eq!(builder, expected_output);
    }
}
