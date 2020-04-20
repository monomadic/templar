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
    let result = templar::parse_str(CODE).unwrap();

    println!("{:#?}", result);
}
