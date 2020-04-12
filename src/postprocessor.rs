use crate::ParseResult;
use crate::*;
use std::collections::HashMap;

/// run pre-processors for a node tree
pub fn run(nodes: Vec<Node>) -> ParseResult<Vec<UnwoundNode>> {
    let locals = extract_variables(&nodes);
    let fns = collect_function_definitions(&nodes)?;

    // let mut fns = HashMap::new(); // imported libs
    // let mut locals = HashMap::new(); // default globals (args?)

    unwind_children(&nodes, locals, fns)
}

fn unwind_children(nodes: &Vec<Node>, locals: HashMap<String, Property>, fns: HashMap<String, Function>) -> ParseResult<Vec<UnwoundNode>> {
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
            let eval_result = evaluate_block(ident, properties, &locals, &unwound_children, &fns)?;

            // unwound_children.extend(eval_result.iter().cloned());
            unwound_nodes.extend(eval_result.iter().cloned());
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

fn evaluate_block(ident: &String, properties: &Vec<Property>, locals: &HashMap<String, Property>, children: &Vec<UnwoundNode>, fns: &HashMap<String, Function>) -> ParseResult<Vec<UnwoundNode>> {
    // if our ident is defined as a function
    if let Some(func) = fns.get(ident) {
        let args = merge_arguments(&func.arguments, properties)?;
        println!("executing fn: {} {:?} {:?} {:?} {:?} {:?}", ident, func.children, func.arguments, locals, properties, args);
        // need to unwind arguments
        return unwind_children(&func.children, args, fns.clone());
    }

    // println!("fn not found: {}", ident);
    Ok(vec![UnwoundNode {
        ident: ident.clone(),
        properties: properties.clone(),
        locals: locals.clone(),
        children: children.clone()
    }])
}

fn extract_variables(nodes: &Vec<Node>) -> HashMap<String, Property> {
    let mut variables: HashMap<String, Property> = HashMap::new();
    println!("extract: {:?}", nodes);

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
        if let Node::FunctionDeclaration { ident, arguments, children } = node {
            fns.insert(ident.to_string(), Function {
                arguments: arguments.clone(),
                children: children.clone()
            });
        };
    }

    Ok(fns)
}
