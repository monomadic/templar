use crate::ParseResult;
use crate::*;
use std::collections::HashMap;

/// run pre-processors for a node tree
pub fn run(nodes: Vec<Node>) -> ParseResult<Vec<UnwoundNode>> {
    let mut fns: HashMap<String, Node> = HashMap::new();

    // collect all function declarations (only valid at the top level)
    for node in nodes.clone() {
        match node.clone() {
            Node::FunctionDeclaration { ident, arguments, children } => {
                println!("found fn declaration: {} {:?}", ident, arguments);
                fns.insert(ident.to_string(), node.clone());
            },
            _ => (),
        }
    }

    // println!("functions: {:?}", fns);

    // 2nd pass: traverse tree, execute functions
    // println!("result: {:?}", unwind_children(&nodes, &fns));
    // for node in nodes {
    //     match node.clone() {
    //         Node::Block { ident, properties, children } => {
    //             println!("--{:?}", unwind(ident, properties, children, &fns));
    //         },
    //         _ => (),
    //     };
    // }

    

    Ok(unwind_children(&nodes, &fns))
}

fn unwind_children(nodes: &Vec<Node>, fns: &HashMap<String, Node>) -> Vec<UnwoundNode> {
    let mut unwound_nodes: Vec<UnwoundNode> = Vec::new();

    //println!("unwinding: {:?}", &nodes);

    for node in nodes {
        // if node is a block,
        if let Node::Block{ ident, properties, children } = node {
            //println!("evaluating: {}", &ident);
            // evaluate its children first
            let unwound_children = unwind_children(&children, &fns);
            //println!("eval result 1: {:?}", &unwound_children);

            // evaluate
            let eval_result = evaluate_block(ident, properties, &unwound_children, &fns);
            //println!("eval result 2: {:?}", &eval_result);

            // unwound_children.extend(eval_result.iter().cloned());
            unwound_nodes.extend(eval_result.iter().cloned());


        }
    };

    unwound_nodes
}

fn evaluate_block(ident: &String, properties: &Vec<Property>, children: &Vec<UnwoundNode>, fns: &HashMap<String, Node>) -> Vec<UnwoundNode> {
    // if our ident is defined as a function
    if let Some(func) = fns.get(ident) {

        // todo: fix this so we don't have to unwrap here.
        if let Node::FunctionDeclaration{ arguments, children, .. } = func {
            println!("executing fn: {} {:?}", &ident, &children);
            return unwind_children(&children, &fns);
        }
    }

    println!("fn not found: {}", ident);
    vec![UnwoundNode {
        ident: ident.clone(),
        properties: properties.clone(),
        children: children.clone()
    }]
}

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

fn extract_variables(nodes: Vec<Node>) -> HashMap<String, Property> {
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
