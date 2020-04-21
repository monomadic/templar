use crate::*;
use std::collections::HashMap;

/// run pre-processors for a node tree. use this if you don't want to manually load pre-existing libs
pub fn run(nodes: Vec<Node>) -> TemplarResult<Vec<UnwoundNode>> {
    let locals = extract_properties(&nodes);
    let overlays = collect_overlay_definitions(&nodes)?;

    unwind_children(&nodes, locals, overlays)
}

/// hook directly into the unwinding process with this overlay instead, for providing external libs + globals.
pub fn unwind_children(nodes: &Vec<Node>, locals: HashMap<String, Property>, overlays: HashMap<String, Overlay>) -> TemplarResult<Vec<UnwoundNode>> {
    let mut unwound_nodes: Vec<UnwoundNode> = Vec::new();
    let mut locals = locals.clone();
    let mut overlays = overlays.clone();

    // upsert new local variable scope
    for (ident, value) in extract_properties(&nodes) {
        locals.insert(ident, value);
    }

    for (ident, value) in collect_overlay_definitions(&nodes)? {
        overlays.insert(ident, value); // note: check for not None as this would override... allowed for now?
    }

    for node in nodes {
        println!("UNWINDING: {:?}", node);
        // if node is a block,
        if let Node::Block{ ident, attributes, children } = node {
            // evaluate its children first
            let unwound_children = unwind_children(&children, locals.clone(), overlays.clone())?;

            let properties = extract_properties(&children);
            println!("PROPS: {:?}{:?}", properties, attributes);

            // let eval_locals = evaluate_variable_scope(&properties, locals);
            let eval_result = unwind(ident, attributes, &properties, &unwound_children, &overlays)?;

            // unwound_children.extend(eval_result.iter().cloned());
            unwound_nodes.push(eval_result);
        };

        // anonymous values get converted into nodes of ident _TEXT with a property .text <eval>
        if let Node::AnonymousProperty(value) = node {
            let mut properties: HashMap<String, Property> = HashMap::new();
            properties.insert("text".into(), value.clone());
            unwound_nodes.push(UnwoundNode {
                ident: "_TEXT".into(),
                attributes: vec![],
                properties,
                children: vec![],
            })
        }
    };

    println!("UNWOUND: {:?}", unwound_nodes);

    Ok(unwound_nodes)
}

// fn merge_arguments(args: &Vec<String>, properties: &Vec<Property>) -> TemplarResult<HashMap<String, Property>> {
//     let mut passed_arguments: HashMap<String, Property> = HashMap::new();

//     for (index, arg) in args.into_iter().enumerate() {
//         let property: Property = properties.get(index).unwrap_or(&Property::QuotedString("ERROR".into())).clone();
//         passed_arguments.insert(arg.clone(), property); // todo: fix
//     }

//     Ok(passed_arguments)
// }

fn unwind(ident: &String, attributes: &Vec<Property>, properties: &HashMap<String, Property>, children: &Vec<UnwoundNode>, overlays: &HashMap<String, Overlay>)
-> TemplarResult<UnwoundNode> {
    let mut unwound_node = UnwoundNode {
        ident: ident.clone(),
        attributes: attributes.clone(),
        properties: properties.clone(),
        children: children.clone()
    };

    println!("CURRENT UNWOUND NODE: {:?}", unwound_node);

    // if our node ident matches any overlays
    if let Some(func) = overlays.get(ident) {
        println!("applying overlay: {:?}", func);

        // find properties on the function
        let mut function_properties = extract_properties(&func.children);

        // now we need to traverse the overlay with a preprocessor that
        // resolves references with arguments eg $1 $2
        let resolved_overlay_children =
            func.children.iter().map(|child| {
                resolve_variable_references_in_overlay(attributes, child)
            }).collect();

        // replace ident
        unwound_node.ident = func.output.clone();
        // expand overlay arguments into properties NOT NECESSARY

        // add properties from inside the overlay
        function_properties.extend(properties.clone().into_iter());

        unwound_node.properties = function_properties;
        // need to unwind children and merge them

        for child in unwind_children(&resolved_overlay_children, unwound_node.properties.clone(), overlays.clone())? {
            unwound_node.children.push(child);
        }
    }

    // println!("fn not found: {}", ident);
    Ok(unwound_node)
}

// fn resolve_variable_references_in_overlay(arguments: &Vec<Property>, nodes: &Vec<Node>) -> Vec<Node> {
//     let nodes = nodes.iter().map(resolve_references).collect();
//     for node in nodes {
//         if let Node::AnonymousProperty(property) = node {
//             if let Property::ArgumentIndex(index) = property {
//                 //println!("-INDEX FOUND: {:?} {:?}", index, arguments.get(index - 1));
//                 //let a = arguments.get(index - 1);
//                 if let Some(property) = arguments.get(index - 1) {
//                     return Node::AnonymousProperty(property);
//                 }
//             }
//         }
//     }
//     nodes.clone()
// }

fn resolve_variable_references_in_overlay(arguments: &Vec<Property>, node: &Node) -> Node {

    // match node {
    //     Node::Block{ children, .. } => {},

    // }

    // children first
    // output_node.children = node.children.into_iter().map(resolve_variable_references_in_overlay(
    //     arguments: &arguments, node: &node
    // ));

    // resolve now
    if let Node::AnonymousProperty(property) = node {
        if let Property::ArgumentIndex(index) = property {
            if let Some(property) = arguments.get(index - 1) {
                return Node::AnonymousProperty(property.clone());
            }
        }
    }

    // else leaf node, return it.
    node.clone()
}



fn extract_properties(nodes: &Vec<Node>) -> HashMap<String, Property> {
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

fn collect_overlay_definitions(nodes: &Vec<Node>) -> TemplarResult<HashMap<String, Overlay>> {
    let mut overlays: HashMap<String, Overlay> = HashMap::new();

    // collect all overlay declarations
    for node in nodes {
        if let Node::Overlay(overlay) = node {
            overlays.insert(overlay.ident.to_string(), overlay.clone());
        };
    }

    Ok(overlays)
}
