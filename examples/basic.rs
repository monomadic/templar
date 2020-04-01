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

const CURRENT: &str = r#"
block 2 1.1
    member example
    second example
        third example
"#;

fn main() {
    println!("----{:#?}", templar::parse(CODE));
    println!("----{:#?}", templar::parse(CURRENT));
    println!("----{:#?}", templar::parse("header title\ntwo three four\nfive six\nseven eight\n"));
    println!("----{:#?}", templar::parse("hello \"b\"\n"));
}
