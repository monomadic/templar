use templar;

const CODE: &str = r#"

print "this is a direct function call."

say "now I am calling a function I will define."

:say .text
    print .text

"#;

fn main() {
    let (input, result) = templar::parser::run(CODE).unwrap();
    let post = templar::postprocessor::run(result).unwrap();

    println!("{:#?}", post);
}
