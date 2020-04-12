use templar;

const CODE: &str = r#"
page "/"
    .title "hi"
    title .title
    
    row
        header .title

style a "b" another
    .title hi
"#;

fn main() {
    let (input, result) = templar::parser::run(CODE).unwrap();
    let post = templar::postprocessor::run(result).unwrap();

    println!("{:#?}", post);
}
