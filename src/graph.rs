use std::collections::{HashMap, HashSet, VecDeque};

/// Tipos diferentes de vertice evitam confusao entre IDs iguais.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Customer(u32),
    Product(u32),
    Category(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Relation {
    Purchase,
    Interest,
    Category,
    Similar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub to: Node,
    pub relation: Relation,
    pub weight: u32,
}

/// Grafo nao direcionado, ponderado, armazenado em lista de adjacencia.
#[derive(Clone, Debug, Default)]
pub struct Graph {
    adjacency: HashMap<Node, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Node) {
        self.adjacency.entry(node).or_default();
    }

    /// Uma aresta por (destino, relacao); nova insercao atualiza peso.
    pub fn link(&mut self, left: Node, right: Node, relation: Relation, weight: u32) {
        self.add_node(left);
        self.add_node(right);
        Self::upsert(self.adjacency.get_mut(&left).expect("vertice existe"), right, relation, weight);
        if left != right {
            Self::upsert(self.adjacency.get_mut(&right).expect("vertice existe"), left, relation, weight);
        }
    }

    fn upsert(edges: &mut Vec<Edge>, to: Node, relation: Relation, weight: u32) {
        if let Some(edge) = edges.iter_mut().find(|edge| edge.to == to && edge.relation == relation) {
            edge.weight = weight;
        } else {
            edges.push(Edge { to, relation, weight });
        }
    }

    pub fn neighbors(&self, node: Node) -> &[Edge] {
        self.adjacency.get(&node).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn vertex_count(&self) -> usize {
        self.adjacency.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency.values().map(Vec::len).sum::<usize>() / 2
    }

    /// BFS com limite de profundidade; cada vertice e visitado uma vez.
    pub fn bfs(&self, start: Node, max_depth: usize) -> Vec<(Node, usize)> {
        if !self.adjacency.contains_key(&start) {
            return Vec::new();
        }
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut discovered = Vec::new();
        visited.insert(start);
        queue.push_back((start, 0_usize));

        while let Some((node, depth)) = queue.pop_front() {
            discovered.push((node, depth));
            if depth >= max_depth {
                continue;
            }
            for edge in self.neighbors(node) {
                if visited.insert(edge.to) {
                    queue.push_back((edge.to, depth + 1));
                }
            }
        }
        discovered
    }
}

#[cfg(test)]
mod tests {
    use super::{Graph, Node, Relation};

    #[test]
    fn bfs_respeita_profundidade_e_visita_unica() {
        let mut graph = Graph::new();
        graph.link(Node::Customer(1), Node::Product(1), Relation::Purchase, 5);
        graph.link(Node::Product(1), Node::Category(1), Relation::Category, 1);
        graph.link(Node::Category(1), Node::Product(2), Relation::Category, 1);
        let nodes = graph.bfs(Node::Customer(1), 2);
        assert_eq!(nodes.len(), 3);
        assert!(!nodes.iter().any(|(n, _)| *n == Node::Product(2)));
    }

    #[test]
    fn arestas_repetidas_nao_sao_duplicadas() {
        let mut graph = Graph::new();
        graph.link(Node::Product(1), Node::Product(2), Relation::Similar, 3);
        graph.link(Node::Product(1), Node::Product(2), Relation::Similar, 4);
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.neighbors(Node::Product(1))[0].weight, 4);
    }
}
