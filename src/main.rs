use megastore::{demo, Product, Recommendation, Store};
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("demo");
    let store = demo::sample_store();
    match command {
        "demo" => show_demo(&store),
        "products" => list_products(&store),
        "customer" => {
            let id = arg_id(&args, 2, 1);
            show_customer(&store, id);
        }
        "product" => {
            let id = arg_id(&args, 2, 101);
            show_product(&store, id);
        }
        "benchmark" => benchmark(),
        "menu" => interactive(store),
        _ => print_help(),
    }
}

fn arg_id(args: &[String], index: usize, default: u32) -> u32 {
    args.get(index).and_then(|val| val.parse().ok()).unwrap_or(default)
}

fn print_help() {
    println!("ConectaStore - comandos:");
    println!("  cargo run -- demo         demonstra o sistema");
    println!("  cargo run -- products     lista o catalogo");
    println!("  cargo run -- customer 1   recomenda para o cliente 1");
    println!("  cargo run -- product 101  recomenda a partir do produto 101");
    println!("  cargo run -- benchmark    mede tempos reais neste computador");
    println!("  cargo run -- menu         cadastro e consulta pelo terminal");
}

fn heading() {
    println!("==============================================================");
    println!(" ConectaStore | MegaStore - Recomendacao baseada em grafos");
    println!("==============================================================");
}

fn list_products(store: &Store) {
    println!("\nCATALOGO DE PRODUTOS");
    for product in store.products_sorted() {
        println!(" ID {:>3} | {:<23} | R$ {:>8.2}", product.id, product.name, product.price_cents as f64 / 100.0);
    }
}

fn print_recommendations(items: &[Recommendation]) {
    if items.is_empty() {
        println!(" Nenhuma recomendacao relevante encontrada.");
    }
    for (i, rec) in items.iter().enumerate() {
        println!(" {}. [{:>3}] {:<23} | score {:>3} | {}", i + 1, rec.product_id, rec.name, rec.score, rec.reason);
    }
}

fn show_customer(store: &Store, id: u32) {
    println!("\nRECOMENDACOES PARA CLIENTE {id}");
    match store.recommend_for_customer(id, 5) {
        Ok(items) => print_recommendations(&items),
        Err(err) => println!(" Erro: {err}"),
    }
}

fn show_product(store: &Store, id: u32) {
    println!("\nPRODUTOS RELACIONADOS AO PRODUTO {id}");
    match store.recommend_for_product(id, 5) {
        Ok(items) => print_recommendations(&items),
        Err(err) => println!(" Erro: {err}"),
    }
}

fn show_demo(store: &Store) {
    heading();
    println!("Dados ficticios para demonstracao academica.");
    println!("Cadastro: {} produtos, {} clientes, {} categorias", store.product_count(), store.customer_count(), store.category_count());
    println!("Grafo: {} vertices e {} arestas", store.graph.vertex_count(), store.graph.edge_count());
    list_products(store);
    show_customer(store, 1);
    show_product(store, 101);
    println!("\nConsultas concluidas; sem recomendacoes repetidas por consulta.");
    println!("Para testes: cargo test | Para desempenho: cargo run --release -- benchmark");
}

fn benchmark() {
    heading();
    println!("BENCHMARK REAL - tempos variam com CPU, sistema e carga");
    println!("Dados sinteticos | 50 consultas de recomendacao para cada tamanho");
    println!("  Produtos | Vertices | Arestas | Tempo medio por consulta (ms)");
    for size in [100_usize, 1_000, 5_000] {
        let store = demo::synthetic_store(size);
        let repetitions = 50;
        let start = Instant::now();
        let mut found = 0_usize;
        for _ in 0..repetitions {
            found += store.recommend_for_customer(1, 5).unwrap().len();
        }
        std::hint::black_box(found);
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0 / repetitions as f64;
        println!(" {:>9} | {:>8} | {:>7} | {:>27.4}", size, store.graph.vertex_count(), store.graph.edge_count(), elapsed_ms);
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return String::new();
    }
    input.trim().to_owned()
}

fn interactive(mut store: Store) {
    heading();
    println!("Modo interativo: dados existem apenas enquanto o processo estiver aberto.");
    loop {
        println!("\n1 - Listar produtos | 2 - Consultar produto | 3 - Cadastrar produto");
        println!("4 - Recomendar por cliente | 5 - Recomendar por produto");
        println!("6 - Resumo do grafo   | 0 - Sair");
        match read_line("Opcao: ").as_str() {
            "1" => list_products(&store),
            "2" => {
                let id: u32 = read_line("ID do produto: ").parse().unwrap_or(0);
                match store.product(id) {
                    Some(p) => println!("Encontrado: {} | R$ {:.2}", p.name, p.price_cents as f64 / 100.0),
                    None => println!("Produto inexistente."),
                }
            }
            "3" => {
                let id: u32 = read_line("Novo ID: ").parse().unwrap_or(0);
                let name = read_line("Nome do produto: ");
                let cents: u32 = read_line("Preco em centavos (ex.: 12990): ").parse().unwrap_or(0);
                let cat: u32 = read_line("Categoria (1=Eletronicos, 2=Escritorio, 3=Livros, 4=Casa): ").parse().unwrap_or(0);
                match store.add_product(Product::new(id, name, cents), &[cat]) {
                    Ok(()) => println!("Produto cadastrado e conectado ao grafo."),
                    Err(err) => println!("Falha no cadastro: {err}"),
                }
            }
            "4" => show_customer(&store, read_line("ID do cliente: ").parse().unwrap_or(0)),
            "5" => show_product(&store, read_line("ID do produto: ").parse().unwrap_or(0)),
            "6" => println!("Grafo: {} vertices e {} arestas.", store.graph.vertex_count(), store.graph.edge_count()),
            "0" => { println!("Encerrado."); break; }
            _ => println!("Opcao invalida."),
        }
    }
}
