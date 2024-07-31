use roxmltree::Node;

pub fn get_unique_node<'a>(node: &Node<'a, 'a>, element_name: &'a str) -> Result<Node<'a, 'a>, &'a str> {
    let mut iter = node.descendants()
        .filter(|n| n.is_element() && n.has_tag_name(element_name));

    let ret_val = iter.next();
    if iter.next().is_some() {
        return Err("element_name is not unique");
    };

    ret_val.ok_or("element_name not found")
}

pub fn get_first_node<'a>(node: &Node<'a, 'a>, element_name: &'a str) -> Result<Node<'a, 'a>, &'a str> {
    node.descendants()
        .find(|n| n.is_element() && n.has_tag_name(element_name))
        .ok_or("element_name not found")
}

pub fn get_unique_node_text<'a>(node: &Node<'a, 'a>, element_name: &'a str) -> Result<&'a str, &'a str> {
    get_unique_node(node, element_name).and_then(|x| x.text().ok_or("element_name does not have text"))
}

pub fn get_nodes<'a>(node: &Node<'a, 'a>, element_name: &'a str) -> Vec<Node<'a, 'a>> {
    node.descendants()
        .filter(|n| n.is_element() && n.has_tag_name(element_name))
        .collect()
}

pub fn get_nodes_text<'a>(node: &Node<'a, 'a>, element_name: &'a str) -> Vec<&'a str> {
    get_nodes(node, element_name)
        .into_iter()
        .filter_map(|x| x.text())
        .collect()
}
