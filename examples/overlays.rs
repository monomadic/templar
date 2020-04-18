use templar;

const CODE: &str = r#"
print "this is a direct block."

say "now I am using an overlay."

:say print .text
    .append "this property is also appended to the callee"

"#;

const HTML: &str = r#"
page "index.html"
    h1 "hello"

:h1 tag
    .text $0
    .background red
    .type "h1"
"#;

fn main() {
    let (output, result) = templar::parser::run(HTML).unwrap();
    let post = templar::postprocessor::run(result).unwrap();

    println!("{:#?}", post);

    println!("output: {}", output);
}
