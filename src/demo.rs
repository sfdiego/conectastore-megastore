use crate::{Product, Store};

/// Dados ficticios e reprodutiveis, exclusivamente para demonstracao academica.
pub fn sample_store() -> Store {
    let mut store = Store::new();
    for (id, name) in [(1, "Eletronicos"), (2, "Escritorio"), (3, "Livros"), (4, "Casa")] {
        store.add_category(id, name).unwrap();
    }
    for (id, name) in [(1, "Ana"), (2, "Bruno"), (3, "Carla")] {
        store.add_customer(id, name).unwrap();
    }
    let products: [(u32, &str, u32, &[u32]); 12] = [
        (101, "Notebook Pro", 489990, &[1, 2]),
        (102, "Mouse Sem Fio", 8990, &[1, 2]),
        (103, "Teclado Mecanico", 27990, &[1, 2]),
        (104, "Hub USB-C", 15990, &[1]),
        (105, "Headset Bluetooth", 34990, &[1]),
        (106, "Cadeira Ergonomica", 109990, &[2, 4]),
        (107, "Mesa Ajustavel", 169990, &[2, 4]),
        (108, "Livro Algoritmos", 11990, &[3, 2]),
        (109, "Livro Rust", 9990, &[3]),
        (110, "Monitor 27 Polegadas", 129990, &[1, 2]),
        (111, "Luminaria LED", 12990, &[2, 4]),
        (112, "Tablet Digital", 249990, &[1]),
    ];
    for (id, name, price, categories) in products {
        store.add_product(Product::new(id, name, price), categories).unwrap();
    }
    for (customer, product) in [(1, 101), (1, 102), (2, 101), (2, 103), (2, 110), (3, 108), (3, 106)] {
        store.connect_purchase(customer, product).unwrap();
    }
    for (customer, category, weight) in [(1, 2, 4), (2, 1, 3), (3, 3, 5)] {
        store.connect_interest(customer, category, weight).unwrap();
    }
    for (first, second, weight) in [(101, 104, 4), (101, 110, 5), (101, 112, 2), (102, 103, 3), (106, 107, 4), (108, 109, 5), (110, 111, 2)] {
        store.connect_similarity(first, second, weight).unwrap();
    }
    store
}

pub fn synthetic_store(n: usize) -> Store {
    let mut store = Store::new();
    for cat in 1..=10 {
        store.add_category(cat, &format!("Categoria {cat}")).unwrap();
    }
    store.add_customer(1, "Cliente benchmark").unwrap();
    for i in 1..=n {
        store.add_product(
            Product::new(i as u32, format!("Produto {i:05}"), 1000 + i as u32),
            &[((i - 1) % 10 + 1) as u32],
        ).unwrap();
    }
    if n >= 1 {
        store.connect_purchase(1, 1).unwrap();
        store.connect_interest(1, 1, 4).unwrap();
    }
    store
}
