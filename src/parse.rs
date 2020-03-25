use crate::*;

/// returns the position of the first non-whitespace character, or None if the line is entirely whitespace.
fn indentation_level(str: &str) -> Option<usize> {
    str.chars().position(|c| !c.is_whitespace())
}

/// parses a string into a node graph
pub fn parse(content:&str) -> ParseResult<Vec<Node>> {
    let lines: Vec<String> = content.lines().map(|s|s.to_string()).collect();
    let mut nodes: Vec<Node> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        // look for first non-whitespace char,
        if let Some(indent) = indentation_level(line) {
            let (_, line_without_whitespace) = line.split_at(indent);


        }
    }

    Ok(nodes)
}
