use megastore::{demo, Product};
use std::collections::HashSet;

#[test]
fn cliente_nao_recebe_produtos_ja_comprados() {
    let store = demo::sample_store();
    let recommendations = store.recommend_for_customer(1, 20).unwrap();
    assert!(!recommendations.is_empty());
    assert!(recommendations.iter().all(|r| r.product_id != 101 && r.product_id != 102));
}

#[test]
fn recomendacoes_nao_repetem_ids() {
    let store = demo::sample_store();
    let recommendations = store.recommend_for_customer(1, 20).unwrap();
    let unique: HashSet<u32> = recommendations.iter().map(|r| r.product_id).collect();
    assert_eq!(unique.len(), recommendations.len());
}

#[test]
fn recomendacoes_estao_ordenadas_por_score() {
    let store = demo::sample_store();
    let recommendations = store.recommend_for_customer(1, 20).unwrap();
    assert!(recommendations.windows(2).all(|window| window[0].score >= window[1].score));
}

#[test]
fn recomendacao_por_produto_retorna_relacionados() {
    let store = demo::sample_store();
    let recommendations = store.recommend_for_product(101, 10).unwrap();
    assert!(recommendations.iter().any(|r| r.product_id == 110));
    assert!(recommendations.iter().all(|r| r.product_id != 101));
}

#[test]
fn cadastro_valida_categoria_e_rejeita_id_repetido() {
    let mut store = demo::sample_store();
    assert!(store.add_product(Product::new(201, "Webcam", 19990), &[1]).is_ok());
    assert!(store.product(201).is_some());
    assert!(store.add_product(Product::new(201, "Repetido", 1000), &[1]).is_err());
    assert!(store.add_product(Product::new(202, "Sem categoria", 1000), &[99]).is_err());
}

#[test]
fn ids_inexistentes_retorna_erro() {
    let store = demo::sample_store();
    assert!(store.recommend_for_customer(999, 5).is_err());
    assert!(store.recommend_for_product(999, 5).is_err());
}

#[test]
fn limite_de_recomendacoes_e_respeitado() {
    let store = demo::sample_store();
    assert_eq!(store.recommend_for_customer(1, 0).unwrap().len(), 0);
    assert!(store.recommend_for_customer(1, 3).unwrap().len() <= 3);
}
