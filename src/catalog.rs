use crate::graph::{Graph, Node, Relation};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price_cents: u32,
}

impl Product {
    pub fn new(id: u32, name: impl Into<String>, price_cents: u32) -> Self {
        Self { id, name: name.into(), price_cents }
    }
}

#[derive(Clone, Debug)]
pub struct Customer {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct Store {
    pub graph: Graph,
    products: HashMap<u32, Product>,
    customers: HashMap<u32, Customer>,
    categories: HashMap<u32, String>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_category(&mut self, id: u32, name: &str) -> Result<(), String> {
        if name.trim().is_empty() || self.categories.contains_key(&id) {
            return Err("Categoria invalida ou ID repetido".to_string());
        }
        self.categories.insert(id, name.to_owned());
        self.graph.add_node(Node::Category(id));
        Ok(())
    }

    pub fn add_customer(&mut self, id: u32, name: &str) -> Result<(), String> {
        if name.trim().is_empty() || self.customers.contains_key(&id) {
            return Err("Cliente invalido ou ID repetido".to_string());
        }
        self.customers.insert(id, Customer { id, name: name.to_owned() });
        self.graph.add_node(Node::Customer(id));
        Ok(())
    }

    pub fn add_product(&mut self, product: Product, category_ids: &[u32]) -> Result<(), String> {
        if product.name.trim().is_empty() || self.products.contains_key(&product.id) {
            return Err("Produto invalido ou ID repetido".to_string());
        }
        if category_ids.is_empty() || category_ids.iter().any(|id| !self.categories.contains_key(id)) {
            return Err("Informe ao menos uma categoria existente".to_string());
        }
        let id = product.id;
        self.products.insert(id, product);
        self.graph.add_node(Node::Product(id));
        for &category in category_ids {
            self.graph.link(Node::Product(id), Node::Category(category), Relation::Category, 1);
        }
        Ok(())
    }

    pub fn connect_purchase(&mut self, customer: u32, product: u32) -> Result<(), String> {
        if !self.customers.contains_key(&customer) || !self.products.contains_key(&product) {
            return Err("Cliente ou produto nao encontrado".to_string());
        }
        self.graph.link(Node::Customer(customer), Node::Product(product), Relation::Purchase, 5);
        Ok(())
    }

    pub fn connect_interest(&mut self, customer: u32, category: u32, weight: u32) -> Result<(), String> {
        if !self.customers.contains_key(&customer) || !self.categories.contains_key(&category) || weight == 0 {
            return Err("Cliente/categoria/peso invalido".to_string());
        }
        self.graph.link(Node::Customer(customer), Node::Category(category), Relation::Interest, weight);
        Ok(())
    }

    pub fn connect_similarity(&mut self, first: u32, second: u32, weight: u32) -> Result<(), String> {
        if !self.products.contains_key(&first) || !self.products.contains_key(&second) || first == second || weight == 0 {
            return Err("Produtos ou peso invalidos".to_string());
        }
        self.graph.link(Node::Product(first), Node::Product(second), Relation::Similar, weight);
        Ok(())
    }

    pub fn product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn customer(&self, id: u32) -> Option<&Customer> {
        self.customers.get(&id)
    }

    pub fn products_sorted(&self) -> Vec<&Product> {
        let mut products: Vec<&Product> = self.products.values().collect();
        products.sort_by_key(|product| product.id);
        products
    }

    pub fn product_count(&self) -> usize {
        self.products.len()
    }

    pub fn customer_count(&self) -> usize {
        self.customers.len()
    }

    pub fn category_count(&self) -> usize {
        self.categories.len()
    }
}
