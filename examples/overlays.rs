use templar;

const HTML: &str = r#"
page "index.html"
    h1 "hello"
    "blah blah lahhdhhd"

:h1 tag content
    .background red
    .type "h1"

    "blah blah blah"
"#;

fn main() {
    let (output, result) = templar::parser::run(HTML).unwrap();
    let post = templar::postprocessor::run(result).unwrap();

    println!("{:#?}", post);

    for node in post {
        print!("{}", node.display(0));
    }

    if !output.is_empty() {
        println!("unparsed remainder: {:#?}", output);
    }
}
