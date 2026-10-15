# ConectaStore — Sistema de recomendação de produtos baseado em grafos

[![Rust CI](https://github.com/sfdiego/conectastore-megastore/actions/workflows/rust.yml/badge.svg)](https://github.com/sfdiego/conectastore-megastore/actions/workflows/rust.yml)

**Autor:** Diego Souza Ferreira | **RA:** 174873  
**Curso:** Engenharia da Computação — UniFecaf  
**Disciplina:** Data Structure Strategy and Implementation  
**Tutor:** Vitor Vieira | **Data do trabalho:** 14/10/2026

> Projeto acadêmico e demonstrativo em Rust. Todos os clientes e produtos são fictícios. Não existe integração com a MegaStore real.

## Objetivo

Propor recomendações por cliente e por produto explorando relações de compra, interesses, categorias e similaridade. O sistema usa **grafo não direcionado e ponderado** em **lista de adjacência**, uma escolha adequada para relações esparsas.

## Tecnologias e estruturas

- **Rust 2021**, apenas biblioteca padrão.
- `HashMap<u32, Product>` para cadastro e busca média O(1).
- `HashMap<Node, Vec<Edge>>` para representar o grafo.
- `VecDeque` para a fila da **busca em largura (BFS)**.
- `HashSet` para visitas únicas, exclusão de compras existentes e categorias.
- `Vec` para ordenar e limitar recomendações.
- `std::time::Instant` para medir desempenho.

## Estrutura do repositório

```text
.
├── .github/workflows/rust.yml
├── .gitignore
├── Cargo.toml
├── DEMONSTRAR_NO_WINDOWS.bat
├── LICENSE
├── README.md
├── src/
│   ├── catalog.rs
│   ├── demo.rs
│   ├── graph.rs
│   ├── lib.rs
│   ├── main.rs
│   └── recommender.rs
└── tests/
    └── integration.rs
```

## Como compilar e executar

Instale [Rust e Cargo](https://rustup.rs/). Na pasta raiz do repositório, use:

```powershell
cargo --version
cargo check --all-targets
cargo run -- demo
cargo run -- products
cargo run -- customer 1
cargo run -- product 101
cargo run -- menu
cargo test --all-targets
cargo run --release -- benchmark
```

No Windows, o arquivo `DEMONSTRAR_NO_WINDOWS.bat` executa compilação, demonstração, testes e benchmark em sequência; o código do script pode ser conferido antes da execução.

### Exemplos

- `cargo run -- demo`: catálogo com 12 produtos, três clientes e quatro categorias, seguido das recomendações.
- `cargo run -- customer 1`: produtos recomendados para a cliente de ID 1, excluindo os já comprados.
- `cargo run -- product 101`: produtos semelhantes ou pertencentes às mesmas categorias do item 101.
- `cargo run -- menu`: consulta e cadastro de produtos em memória. Reiniciar o programa restaura a massa original.

## Arquitetura e algoritmo

`catalog.rs` administra cadastros e ligações; `graph.rs` gerencia a lista de adjacência e a BFS; `recommender.rs` calcula os escores; `demo.rs` define os dados fictícios; `main.rs` contém CLI e benchmark.

A recomendação por **cliente** percorre até três arestas (BFS), elimina itens já adquiridos e pontua os candidatos: categoria de interesse (15 × peso), similaridade com compra anterior (10 × peso), categoria de compra anterior (5 pontos por compra compartilhada) e bônus de proximidade. A recomendação por **produto** percorre até duas arestas e pontua similaridade declarada (12 × peso), categoria comum (8 pontos) e proximidade. O resultado é ordenado por pontuação decrescente, nome e ID; uma recomendação por produto evita repetições.

## Testes e validação

```powershell
cargo test --all-targets
```

Há dois testes unitários de grafos e sete testes de integração, cobrindo BFS, atualização de arestas, exclusão de compras, ordenação, prevenção de duplicatas, IDs inexistentes, cadastro válido e limites. **Execução local documentada em 14/10/2026 (Windows): sete testes de integração aprovados e nenhum reprovado; a sequência de validação avançou até o benchmark.** Os testes também são executados pelo workflow do GitHub Actions.

## Benchmark real — execução local de 14/10/2026

Com `cargo run --release -- benchmark`, 50 consultas são repetidas para cada conjunto sintético:

| Produtos | Vértices | Arestas | Tempo médio por consulta |
| ---: | ---: | ---: | ---: |
| 100 | 111 | 102 | **0,0097 ms** |
| 1.000 | 1.011 | 1.002 | **0,0363 ms** |
| 5.000 | 5.011 | 5.002 | **0,1953 ms** |

Medições reais apresentadas pelo terminal do estudante. Resultados dependem de CPU, sistema operacional, compilação e carga do computador; não constituem garantia de desempenho em produção.

## Complexidade e limites

Uma lista de adjacência ocupa O(V + E), contra O(V²) de uma matriz. A BFS custa O(V + E) no pior caso e O(V) de memória auxiliar. A consulta por chave em HashMap custa O(1) em média. Classificar k candidatos custa O(k log k). O cálculo dos escores examina categorias, compras e vizinhos, podendo acrescentar custo relevante. O protótipo não implementa persistência, autenticação, recomendação por aprendizado de máquina nem engenharia de escala para milhões de produtos. Uma evolução poderia usar persistência, índices invertidos, cache e atualização incremental.

## Vídeo pitch e relatório

O vídeo pitch da disciplina, com duração inferior a quatro minutos, está publicado no YouTube em modo não listado (acessível a quem possui o link):

**[Assistir ao vídeo pitch do ConectaStore](https://youtu.be/6VQ0yImVMlM)**

O relatório técnico em PDF é o outro documento de entrega da disciplina. A data de referência da entrega acadêmica é **15/10/2026**.

## Referências

- [Rust Standard Library — collections](https://doc.rust-lang.org/std/collections/)
- [Rust — HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Rust — VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- [Rust — Instant](https://doc.rust-lang.org/std/time/struct.Instant.html)
- UniFecaf, material da disciplina Data Structure Strategy and Implementation, unidades II e IV.

## Licença

MIT — ver [LICENSE](LICENSE).
