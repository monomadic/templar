use templar;

const CODE: &str = r#"
print "this is a direct block."

say "now I am using an overlay."

:say print .text
    .append "this property is also appended to the callee"

"#;

fn main() {
    let (input, result) = templar::parser::run(CODE).unwrap();
    let post = templar::postprocessor::run(result).unwrap();

    println!("{:#?}", post);
}
