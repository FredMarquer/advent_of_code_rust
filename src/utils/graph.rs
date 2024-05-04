use itertools::Itertools;

#[derive(Clone)]
pub struct Graph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

#[allow(dead_code)]
impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn create_node(&mut self, value: N) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            id,
            value,
            connections: Vec::new(),
        });
        id
    }

    pub fn create_edge(&mut self, from: usize, to: usize, is_bidirectional: bool, value: E) -> usize {
        let edge_id = self.edges.len();
        self.edges.push(Edge {
            id: edge_id,
            from,
            to,
            is_bidirectional,
            has_been_removed: false,
            value,
        });

        let from_node = &mut self.nodes[from];
        from_node.connections.push(Connection {
            to_node_id: to,
            edge_id,
        });

        if is_bidirectional {
            let to_node = &mut self.nodes[to];
            to_node.connections.push(Connection {
                to_node_id: from,
                edge_id,
            });
        }

        edge_id
    }

    pub fn remove_edge(&mut self, edge_id: usize) {
        let edge = &mut self.edges[edge_id];
        edge.has_been_removed = true;

        let from_node = &mut self.nodes[edge.from];
        let index = from_node.connections.iter().position(|connection| connection.edge_id == edge_id).unwrap();
        from_node.connections.remove(index);

        if edge.is_bidirectional {
            let to_node = &mut self.nodes[edge.to];
            let index = to_node.connections.iter().position(|connection| connection.edge_id == edge_id).unwrap();
            to_node.connections.remove(index);
        }


    }

    pub fn remove_edge_from_to(&mut self, from: usize, to: usize) {
        let from_node = &mut self.nodes[from];
        let Some(connection) = from_node.connections.iter().find(|connection| connection.to_node_id() == to) else {
            return;
        };
        let edge_id = connection.edge_id;
        self.remove_edge(edge_id);
    }

    pub fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    pub fn edges_len(&self) -> usize {
        self.edges.len()
    }

    pub fn get_node(&self, node_id: usize) -> &Node<N> {
        &self.nodes[node_id]
    }

    pub fn get_node_mut(&mut self, node_id: usize) -> &mut Node<N> {
        &mut self.nodes[node_id]
    }

    pub fn get_edge(&self, edge_id: usize) -> &Edge<E> {
        &self.edges[edge_id]
    }

    pub fn get_edge_mut(&mut self, edge_id: usize) -> &mut Edge<E> {
        &mut self.edges[edge_id]
    }

    pub fn nodes_iter(&self) -> std::slice::Iter<'_, Node<N>> {
        self.nodes.iter()
    }

    pub fn nodes_iter_mut(&mut self) -> std::slice::IterMut<'_, Node<N>> {
        self.nodes.iter_mut()
    }

    pub fn find_edge(&self, from: usize, to: usize) -> Option<usize> {
        Some(
            self.nodes[from].connections
                .iter()
                .find(|connection| connection.to_node_id == to)?
                .edge_id
        )
    }

    pub fn edges_iter(&self) -> std::slice::Iter<'_, Edge<E>> {
        self.edges.iter()
    }

    pub fn edges_iter_mut(&mut self) -> std::slice::IterMut<'_, Edge<E>> {
        self.edges.iter_mut()
    }

    pub fn convert<NN, EE>(&self, map_node: impl Fn(&N) -> NN, map_edge: impl Fn(&E) -> EE) -> Graph<NN, EE> {
        let mut new_graph = Graph {
            nodes: Vec::with_capacity(self.nodes.len()),
            edges: Vec::with_capacity(self.edges.len()),
        };

        for node in self.nodes.iter() {
            new_graph.nodes.push(Node {
                id: node.id,
                value: map_node(&node.value),
                connections: node.connections.clone(),
            });
        }

        for edge in self.edges.iter() {
            new_graph.edges.push(Edge {
                id: edge.id,
                from: edge.from,
                to: edge.to,
                is_bidirectional: edge.is_bidirectional,
                has_been_removed: edge.has_been_removed,
                value: map_edge(&edge.value),
            });
        }

        new_graph
    }

    pub fn convert_default<NN: Default, EE: Default>(&self) -> Graph<NN, EE> {
        self.convert(
            |_| NN::default(),
            |_| EE::default(),
        )
    }
}

#[derive(Clone)]
pub struct Node<N> {
    id: usize,
    value: N,
    connections: Vec<Connection>,
}

#[allow(dead_code)]
impl<N> Node<N> {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn value(&self) -> &N {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut N {
        &mut self.value
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }
}

#[derive(Clone)]
pub struct Connection {
    to_node_id: usize,
    edge_id: usize,
}

impl Connection {
    pub fn to_node_id(&self) -> usize {
        self.to_node_id
    }

    pub fn edge_id(&self) -> usize {
        self.edge_id
    }
}

#[derive(Clone)]
pub struct Edge<E> {
    id: usize,
    from: usize,
    to: usize,
    is_bidirectional: bool,
    has_been_removed: bool,
    value: E,
}

#[allow(dead_code)]
impl<E> Edge<E> {
    pub fn id(&self) -> usize {
        self.id
    }
    
    pub fn from(&self) -> usize {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn is_bidirectional(&self) -> bool {
        self.is_bidirectional
    }

    pub fn has_been_removed(&self) -> bool {
        self.has_been_removed
    }

    pub fn value(&self) -> &E {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut E {
        &mut self.value
    }
}