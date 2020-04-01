use templar;

const code: &str = r#"
page "/" 2 4.56
    .title "hi"
    title .title
    
    row
        header .title

style a "b" another
    .title hi
"#;

const current: &str = r#"
block symbol
    member example
    second example
        third example
"#;

fn main() {
    println!("----{:#?}", templar::parse(current));
    // println!("----{:#?}", templar::parse("header title\ntwo three four\nfive six\nseven eight\n"));
}
