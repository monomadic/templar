use templar;

const code: &str = r#"
page first second
    title .title
    
    row
        header .title

style a "b" another
    .title hi
"#;

fn main() {
    println!("----{:?}", templar::parse(code));
}
