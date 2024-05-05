use roxmltree::Node;

pub fn get_unique_node<'a>(node: Node<'a, 'a>, element_name: &'a str) -> Option<Node<'a, 'a>> {
    node.descendants()
        .find(|n| n.is_element() && n.has_tag_name(element_name))
}

pub fn get_unique_node_text<'a>(node: Node<'a, 'a>, element_name: &'a str) -> Option<&'a str> {
    get_unique_node(node, element_name)?.text()
}

pub fn get_nodes<'a>(node: Node<'a, 'a>, element_name: &'a str) -> Vec<Node<'a, 'a>> {
    node.descendants()
        .filter(|n| n.is_element() && n.has_tag_name(element_name))
        .collect()
}

pub fn get_nodes_text<'a>(node: Node<'a, 'a>, element_name: &'a str) -> Vec<&'a str> {
    get_nodes(node, element_name)
        .into_iter()
        .filter_map(|x| x.text())
        .collect()
}
