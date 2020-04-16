use crate::ParseResult;
use crate::*;
use std::collections::HashMap;

/// run pre-processors for a node tree. use this if you don't want to manually load pre-existing libs
pub fn run(nodes: Vec<Node>) -> ParseResult<Vec<UnwoundNode>> {
    let locals = extract_variables(&nodes);
    let overlays = collect_overlay_definitions(&nodes)?;

    println!("locals: {:#?}\nfns:{:?}", locals, overlays);
    unwind_children(&nodes, locals, overlays)
}

/// hook directly into the unwinding process with this overlay instead, for providing external libs + globals.
pub fn unwind_children(nodes: &Vec<Node>, locals: HashMap<String, Property>, overlays: HashMap<String, Overlay>) -> ParseResult<Vec<UnwoundNode>> {
    let mut unwound_nodes: Vec<UnwoundNode> = Vec::new();
    let mut locals = locals.clone();
    let mut overlays = overlays.clone();

    // upsert new local variable scope
    for (ident, value) in extract_variables(&nodes) {
        locals.insert(ident, value);
    }

    for (ident, value) in collect_overlay_definitions(&nodes)? {
        overlays.insert(ident, value); // note: check for not None as this would override... allowed for now?
    }

    for node in nodes {
        // if node is a block,
        if let Node::Block{ ident, attributes, children } = node {
            // evaluate its children first
            let unwound_children = unwind_children(&children, locals.clone(), overlays.clone())?;

            // let eval_locals = evaluate_variable_scope(&properties, locals);
            let eval_result = unwind(ident, attributes, &locals, &unwound_children, &overlays)?;

            // unwound_children.extend(eval_result.iter().cloned());
            // unwound_nodes.extend(eval_result.iter().cloned());
            unwound_nodes.push(eval_result);
        }
    };

    Ok(unwound_nodes)
}

fn merge_arguments(args: &Vec<String>, properties: &Vec<Property>) -> ParseResult<HashMap<String, Property>> {
    let mut passed_arguments: HashMap<String, Property> = HashMap::new();

    for (index, arg) in args.into_iter().enumerate() {
        let property: Property = properties.get(index).unwrap_or(&Property::QuotedString("ERROR".into())).clone();
        passed_arguments.insert(arg.clone(), property); // todo: fix
    }

    Ok(passed_arguments)
}

fn unwind(ident: &String, properties: &Vec<Property>, locals: &HashMap<String, Property>, children: &Vec<UnwoundNode>, overlays: &HashMap<String, Overlay>)
-> ParseResult<UnwoundNode> {
    let mut unwound_node = UnwoundNode {
        ident: ident.clone(),
        properties: properties.clone(),
        locals: locals.clone(),
        children: children.clone()
    };

    // if our node ident matches any overlays
    if let Some(func) = overlays.get(ident) {
        unwound_node.ident = func.output.clone();
        // expand overlay arguments into properties
        // todo: change locals to properties
        unwound_node.locals = merge_arguments(&func.arguments, properties)?;
        // need to unwind children and merge them
        // return unwind_children(&func.children, args, overlays.clone());
    }

    // println!("fn not found: {}", ident);
    Ok(unwound_node)
}

fn extract_variables(nodes: &Vec<Node>) -> HashMap<String, Property> {
    let mut variables: HashMap<String, Property> = HashMap::new();

    for node in nodes {
        match node.clone() {
            Node::Assignment { ident, value } => {
                variables.insert(ident.to_string(), value.clone());
            },
            _ => (),
        }
    }

    return variables;
}

fn collect_overlay_definitions(nodes: &Vec<Node>) -> ParseResult<HashMap<String, Overlay>> {
    let mut overlays: HashMap<String, Overlay> = HashMap::new();

    // collect all overlay declarations
    for node in nodes {
        if let Node::Overlay(overlay) = node {
            overlays.insert(overlay.ident.to_string(), overlay.clone());
        };
    }

    Ok(overlays)
}
