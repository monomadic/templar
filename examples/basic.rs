use templar;

const code: &str = r#"
empty

page "/" 2 4.56
    title .title
    
    row
        header .title

style a "b" another
    .title hi
"#;

fn main() {
    println!("----{:?}", templar::parse(code));
}
