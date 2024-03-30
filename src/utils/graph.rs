pub struct Graph<T> {
    nodes: Vec<Node<T>>
}

#[allow(dead_code)]
impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new()
        }
    }

    pub fn create_node(&mut self, value: T) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            id: self.nodes.len(),
            value,
            connections: Vec::new(),
        });
        id
    }

    pub fn create_connection(&mut self, from: usize, to: usize, is_bidirectional: bool) {
        if let Some(from_node) = self.get_node_mut(from) {
            from_node.connections.push(to);
        }

        if is_bidirectional {
            if let Some(to_node) = self.get_node_mut(to) {
                to_node.connections.push(from);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node(&self, node_id: usize) -> Option<&Node<T>> {
        if node_id < self.nodes.len() {
            return Some(&self.nodes[node_id]);
        }
        None
    }

    pub fn get_node_mut(&mut self, node_id: usize) -> Option<&mut Node<T>> {
        if node_id < self.nodes.len() {
            return Some(&mut self.nodes[node_id]);
        }
        None
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Node<T>> {
        self.nodes.iter()
    }
}

pub struct Node<T> {
    id: usize,
    value: T,
    connections: Vec<usize>,
}

#[allow(dead_code)]
impl<T> Node<T> {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn connections(&self) -> &[usize] {
        &self.connections
    }

    pub fn connections_mut(&mut self) -> &mut Vec<usize> {
        &mut self.connections
    }
}