use crate::ParseResult;
use crate::*;
use std::collections::HashMap;

/// run pre-processors for a node tree. use this if you don't want to manually load pre-existing libs
pub fn run(nodes: Vec<Node>) -> ParseResult<Vec<UnwoundNode>> {
    let locals = extract_variables(&nodes);
    let fns = collect_function_definitions(&nodes)?;

    println!("locals: {:#?}\nfns:{:?}", locals, fns);
    unwind_children(&nodes, locals, fns)
}

/// hook directly into the unwinding process with this function instead, for providing external libs + globals.
pub fn unwind_children(nodes: &Vec<Node>, locals: HashMap<String, Property>, fns: HashMap<String, Function>) -> ParseResult<Vec<UnwoundNode>> {
    let mut unwound_nodes: Vec<UnwoundNode> = Vec::new();
    let mut locals = locals.clone();
    let mut fns = fns.clone();

    // upsert new local variable scope
    for (ident, value) in extract_variables(&nodes) {
        locals.insert(ident, value);
    }

    for (ident, value) in collect_function_definitions(&nodes)? {
        fns.insert(ident, value); // note: check for not None as this would override... allowed for now?
    }

    for node in nodes {
        // if node is a block,
        if let Node::Block{ ident, properties, children } = node {
            // evaluate its children first
            let unwound_children = unwind_children(&children, locals.clone(), fns.clone())?;

            // let eval_locals = evaluate_variable_scope(&properties, locals);
            let eval_result = unwind(ident, properties, &locals, &unwound_children, &fns)?;

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

fn unwind(ident: &String, properties: &Vec<Property>, locals: &HashMap<String, Property>, children: &Vec<UnwoundNode>, fns: &HashMap<String, Function>)
-> ParseResult<UnwoundNode> {
    let mut unwound_node = UnwoundNode {
        ident: ident.clone(),
        properties: properties.clone(),
        locals: locals.clone(),
        children: children.clone()
    };

    // if our node ident matches any overlays
    if let Some(func) = fns.get(ident) {
        unwound_node.ident = func.output.clone();
        // expand overlay arguments into properties
        // todo: change locals to properties
        unwound_node.locals = merge_arguments(&func.arguments, properties)?;
        // need to unwind children and merge them
        // return unwind_children(&func.children, args, fns.clone());
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

fn collect_function_definitions(nodes: &Vec<Node>) -> ParseResult<HashMap<String, Function>> {
    let mut fns: HashMap<String, Function> = HashMap::new();

    // collect all function declarations
    for node in nodes {
        if let Node::FunctionDeclaration(function) = node {
            fns.insert(function.ident.to_string(), function.clone());
        };
    }

    Ok(fns)
}
