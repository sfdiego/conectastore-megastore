use crate::catalog::Store;
use crate::graph::{Node, Relation};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recommendation {
    pub product_id: u32,
    pub name: String,
    pub score: u32,
    pub reason: String,
}

impl Store {
    /// Descobre candidatos por BFS (3 saltos) e os pontua por interesse,
    /// similaridade e categorias comuns a compras anteriores.
    pub fn recommend_for_customer(&self, customer_id: u32, limit: usize) -> Result<Vec<Recommendation>, String> {
        if self.customer(customer_id).is_none() {
            return Err("Cliente nao encontrado".to_string());
        }
        let start = Node::Customer(customer_id);
        let bought: HashSet<u32> = self.graph.neighbors(start)
            .iter()
            .filter(|edge| edge.relation == Relation::Purchase)
            .filter_map(|edge| match edge.to { Node::Product(id) => Some(id), _ => None })
            .collect();
        let interests: HashMap<u32, u32> = self.graph.neighbors(start)
            .iter()
            .filter(|edge| edge.relation == Relation::Interest)
            .filter_map(|edge| match edge.to { Node::Category(id) => Some((id, edge.weight)), _ => None })
            .collect();

        let mut recommendations = Vec::new();
        for (node, depth) in self.graph.bfs(start, 3) {
            let id = match node { Node::Product(id) => id, _ => continue };
            if bought.contains(&id) { continue; }
            let product = match self.product(id) { Some(p) => p, None => continue };
            let candidate_categories = self.categories_of(id);
            let mut score = 0_u32;
            let mut reasons = Vec::new();

            for cat in &candidate_categories {
                if let Some(weight) = interests.get(cat) {
                    score += weight.saturating_mul(15);
                    reasons.push("categoria de interesse");
                }
            }
            for purchased_id in &bought {
                if candidate_categories.iter().any(|cat| self.categories_of(*purchased_id).contains(cat)) {
                    score += 5;
                    reasons.push("categoria de compra anterior");
                }
                for edge in self.graph.neighbors(Node::Product(*purchased_id)) {
                    if edge.relation == Relation::Similar && edge.to == Node::Product(id) {
                        score += edge.weight.saturating_mul(10);
                        reasons.push("produto semelhante a uma compra");
                    }
                }
            }
            if score == 0 { continue; }
            score += 4_u32.saturating_sub(depth as u32);
            reasons.sort_unstable();
            reasons.dedup();
            recommendations.push(Recommendation {
                product_id: id,
                name: product.name.clone(),
                score,
                reason: reasons.join("; "),
            });
        }
        sort_and_limit(&mut recommendations, limit);
        Ok(recommendations)
    }

    /// Busca produtos ligados por similaridade ou categoria (BFS de 2 saltos).
    pub fn recommend_for_product(&self, product_id: u32, limit: usize) -> Result<Vec<Recommendation>, String> {
        if self.product(product_id).is_none() {
            return Err("Produto nao encontrado".to_string());
        }
        let categories: HashSet<u32> = self.categories_of(product_id).into_iter().collect();
        let mut recommendations = Vec::new();
        for (node, depth) in self.graph.bfs(Node::Product(product_id), 2) {
            let candidate_id = match node { Node::Product(id) if id != product_id => id, _ => continue };
            let product = match self.product(candidate_id) { Some(p) => p, None => continue };
            let mut score = 0_u32;
            let mut reasons = Vec::new();
            for edge in self.graph.neighbors(Node::Product(product_id)) {
                if edge.relation == Relation::Similar && edge.to == node {
                    score += edge.weight.saturating_mul(12);
                    reasons.push("similaridade declarada");
                }
            }
            if self.categories_of(candidate_id).iter().any(|cat| categories.contains(cat)) {
                score += 8;
                reasons.push("categoria em comum");
            }
            if score == 0 { continue; }
            score += 3_u32.saturating_sub(depth as u32);
            recommendations.push(Recommendation {
                product_id: candidate_id,
                name: product.name.clone(),
                score,
                reason: reasons.join("; "),
            });
        }
        sort_and_limit(&mut recommendations, limit);
        Ok(recommendations)
    }

    fn categories_of(&self, product: u32) -> Vec<u32> {
        self.graph.neighbors(Node::Product(product)).iter()
            .filter(|edge| edge.relation == Relation::Category)
            .filter_map(|edge| match edge.to { Node::Category(id) => Some(id), _ => None })
            .collect()
    }
}

fn sort_and_limit(recommendations: &mut Vec<Recommendation>, limit: usize) {
    recommendations.sort_by(|a, b| b.score.cmp(&a.score)
        .then_with(|| a.name.cmp(&b.name))
        .then_with(|| a.product_id.cmp(&b.product_id)));
    recommendations.truncate(limit);
}
