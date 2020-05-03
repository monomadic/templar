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
        // if node is a block,
        if let Node::Block{ ident, attributes, children } = node {
            // evaluate its children first
            let unwound_children = unwind_children(&children, locals.clone(), overlays.clone())?;

            let properties = extract_properties(&children);

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

    Ok(unwound_nodes)
}

fn unwind(ident: &String, attributes: &Vec<Property>, properties: &HashMap<String, Property>, children: &Vec<UnwoundNode>, overlays: &HashMap<String, Overlay>)
-> TemplarResult<UnwoundNode> {
    let mut unwound_node = UnwoundNode {
        ident: ident.clone(),
        attributes: attributes.clone(),
        properties: properties.clone(),
        children: children.clone()
    };

    // if our node ident matches any overlays
    if let Some(func) = overlays.get(ident) {

        // find properties on the overlay
        let mut overlay_properties = extract_properties(&func.children);

        let original_children = unwound_node.children;

        unwound_node.children = Vec::new();

        // now we need to traverse the overlay with a preprocessor that
        // resolves references to $0
        // unwound_node.children =
        //     func.children.iter().map(|child| {
        //         resolve_content_references_in_overlay(attributes, child)
        //     }).collect();

        // replace ident
        unwound_node.ident = func.output.clone();
        // expand overlay arguments into properties NOT NECESSARY

        // add properties from inside the overlay
        overlay_properties.extend(properties.clone().into_iter());

        unwound_node.properties = overlay_properties;
        // need to unwind children and merge them

        for child in unwind_children(&func.children, unwound_node.properties.clone(), overlays.clone())? {
            let mut newchild = child.clone();

            newchild.properties = resolve_references_in_properties(&child.properties, attributes);

            unwound_node.children.push(newchild);
        }

        unwound_node.children = insert_content_in_overlay(&unwound_node.children, &original_children);
    }

    Ok(unwound_node)
}

fn insert_content_in_overlay(children: &Vec<UnwoundNode>, content: &Vec<UnwoundNode>) -> Vec<UnwoundNode> {
    let mut resolved_children: Vec<UnwoundNode> = Vec::new();

    for child in children {
        if let Some(text_property) = child.properties.get("text") {
            if let Property::ArgumentIndex(index) = text_property {
                if *index == 0 {
                    resolved_children = [resolved_children.clone(), content.clone()].concat();
                    break;
                }
            }
        }

        resolved_children.push(child.clone());
    };

    resolved_children
}

fn resolve_references_in_properties(properties: &HashMap<String, Property>, arguments: &Vec<Property>) -> HashMap<String, Property> {
    properties.iter().map(|(k, v)| {
        if let Property::ArgumentIndex(index) = v {
            // temporary fix for 0 indexes
            if *index == 0 {
                return (k.clone(), v.clone());
            }
            // end temporary fix
            if let Some(property) = arguments.get(index - 1) {
                return (k.clone(), property.clone());
            }
        }
        (k.clone(), v.clone())
    }).collect()
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
