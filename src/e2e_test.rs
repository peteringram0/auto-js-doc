#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_basic() {
        let source_code = r#"
            function testNoExport(param1: string, param2?: boolean) {

            }
            
            export function testExport(param1: string) {
            
            }
        "#;

        let expected_output = r#"
            /**
             * testNoExport
             *
             * @param {string} param1 - 
             * @param {boolean} [param2] - 
             */
            function testNoExport(param1: string, param2?: boolean) {

            }
            
            /**
             * testExport
             *
             * @param {string} param1 - 
             */
            export function testExport(param1: string) {
            
            }
        "#;

        let updated_code = process(source_code);
        println!("{}", updated_code);
        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_exported() {
        let source_code = r#"
            export function testExport(param1: string) {
            
            }
        "#;

        let expected_output = r#"
            /**
             * testExport
             *
             * @param {string} param1 - 
             */
            export function testExport(param1: string) {
            
            }
        "#;

        let updated_code = process(source_code);
        println!("{}", updated_code);
        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_defaults() {
        let source_code = r#"
            export function test(param1: string = "default value") {
            
            }
        "#;

        let expected_output = r#"
            /**
             * test
             *
             * @param {string} param1="default value" - 
             */
            export function test(param1: string = "default value") {
            
            }
        "#;

        let updated_code = process(source_code);
        println!("{}", updated_code);
        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_inferred_types() {
        let source_code = r#"
            export function test(param2 = true) {

            }
        "#;

        let expected_output = r#"
            /**
             * test
             *
             * @param {unknown} param2 - 
             */
            export function test(param2 = true) {

            }
        "#;

        let updated_code = process(source_code);
        println!("{}", updated_code);
        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_union_type() {
        let source_code = r#"
            export function test(param: string | number) {

            }
        "#;

        let expected_output = r#"
            /**
             * test
             *
             * @param {string | number} param - 
             */
            export function test(param: string | number) {

            }
        "#;

        let updated_code = process(source_code);
        println!("{}", updated_code);
        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_export_type() {
        let source_code = r#"
            export function test(): Promise<string> {

            }
            function testTwo(): Promise<string> {

            }
        "#;

        let expected_output = r#"
            /**
             * test
             *
             * @returns {Promise<string>} 
             */
            export function test(): Promise<string> {

            }
        "#;

        let updated_code = process(source_code);
        println!("{}", updated_code);
        assert_eq!(updated_code, expected_output);
    }

    #[test]
    fn test_class() {
        let source_code = r#"
            class A {
                
                constructor(private readonly a: string) {
                    // TODO
                }
                
                testNoExport(param1: string, param2?: boolean) {
                    // TODO
                }

                public aa() {
                    // TODO
                }

                private b() {
                    // TODO
                }

                static c() {
                    // TODO
                }
            }
        "#;

        let expected_output = r#"
            class A {
                
                /**
                 * constructor
                 *
                 * @param {string} a - 
                 */
                constructor(private readonly a: string) {
                    // TODO
                }
                
                /**
                 * testNoExport
                 *
                 * @param {string} param1 - 
                 * @param {boolean} [param2] - 
                 */
                testNoExport(param1: string, param2?: boolean) {
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

                /**
                 * c
                 */
                static c() {
                    // TODO
                }
            }
        "#;

        let updated_code = process(source_code);
        println!("a {}", updated_code);
        assert_eq!(updated_code, expected_output);
    }
}
