use templar;

const code: &str = r#"
layout page-section
    block row
        h1 .headline
"#;

fn main() {
    println!("{:?}", templar::parse(code));
}
