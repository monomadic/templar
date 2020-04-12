use crate::ParseResult;
use crate::*;
use std::collections::HashMap;

/// run pre-processors for a node tree
pub fn run(nodes: Vec<Node>) -> ParseResult<Vec<UnwoundNode>> { // provide fns also
    let mut fns: HashMap<String, Function> = HashMap::new();
    let locals = extract_variables(&nodes);
    println!("locals: {:?}", &locals);

    // collect all function declarations (only valid at the top level)
    for node in nodes.clone() {
        match node.clone() {
            Node::FunctionDeclaration { ident, arguments, children } => {
                println!("found fn declaration: {} {:?}", ident, arguments);
                fns.insert(ident.to_string(), Function {
                    arguments,
                    children
                });
            },
            _ => (),
        }
    }

    Ok(unwind_children(&nodes, &locals, &fns))
}

fn unwind_children(nodes: &Vec<Node>, locals: &HashMap<String, Property>, fns: &HashMap<String, Function>) -> Vec<UnwoundNode> {
    let mut unwound_nodes: Vec<UnwoundNode> = Vec::new();

    for node in nodes {
        // if node is a block,
        if let Node::Block{ ident, properties, children } = node {
            // evaluate its children first
            let unwound_children = unwind_children(&children, &locals, &fns);
            let eval_result = evaluate_block(ident, properties, locals, &unwound_children, fns);

            // unwound_children.extend(eval_result.iter().cloned());
            unwound_nodes.extend(eval_result.iter().cloned());
        }
    };

    unwound_nodes
}

fn evaluate_block(ident: &String, properties: &Vec<Property>, locals: &HashMap<String, Property>, children: &Vec<UnwoundNode>, fns: &HashMap<String, Function>) -> Vec<UnwoundNode> {
    // if our ident is defined as a function
    if let Some(func) = fns.get(ident) {
        println!("executing fn: {} {:?} {:?} {:?}", ident, func.children, func.arguments, locals);
        return unwind_children(&func.children, &locals, fns);
    }

    println!("fn not found: {}", ident);
    vec![UnwoundNode {
        ident: ident.clone(),
        properties: properties.clone(),
        locals: locals.clone(),
        children: children.clone()
    }]
}

// fn resolve_fn_arguments(arguments: Vec<String>, locals: Vec<Property>) -> Result<Vec<Property>, Box<dyn std::error::Error>> {
//     let mut locals = Vec::new();
//     for arg in arguments {
//         let local: Property = locals.into_iter().find(|p: Property| p.ident == arg);
//         locals.push(local);
//     }
//     Ok(locals)
// }

// fn unwind_node(ident: String, properties: Vec<Property>, children: Vec<Node>, fns: &HashMap<String, Node>) -> Option<UnwoundNode> {
//     let mut unwound_children: Vec<UnwoundNode> = Vec::new();

//     // if the node has blocks, execute those first
//     for child in children.clone() {
//         match child {
//             Node::Block { ident, properties, children } => {
//                 unwound_children.push(
//                     unwind(ident, properties, children, &fns)
//                 );
//             },
//             _ =>()
//         }
//     };

//     // collect any attribute nodes and reconcile them against properties of the block
//     let variables = extract_variables(children);
//     // todo: reconcile properties to variables

//     // if there's a function defined for this node, execute that
//     if let Some(func) = fns.get(&ident) {
//         if let Node::FunctionDeclaration{ arguments, children, .. } = func {

//             for child in children {
//                 if let Node::Block{ ident, properties, children } = child {
//                     unwound_children.push(
//                         unwind(ident.clone(), properties.clone(), children.clone(), &fns)
//                     );
//                 }
//             }

//             // and return the resultant node in place.
//             // return unwind(ident.clone(), properties.clone(), children.clone(), &fns);

//             // return UnwoundNode {
//             //     ident: ident.clone(),
//             //     properties: properties.clone(),
//             //     children: unwound_children
//             // };
//         }
//     }

//     Some(UnwoundNode {
//         ident,
//         properties,
//         children: unwound_children
//     })
// }

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
