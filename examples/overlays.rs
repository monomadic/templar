use templar;

const HTML: &str = r#"
page "index.html"
    h1 "hello"
    "blah blah lahhdhhd"

:h1 tag
    .background red
    .type "h1"
    "blah blah blah"
"#;

fn main() {
    let (output, result) = templar::parser::run(HTML).unwrap();
    let post = templar::postprocessor::run(result).unwrap();

    println!("{:#?}", post);
    println!("output: {}", output);
}
