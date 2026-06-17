use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::time::Instant;

// Vincula diretamente os símbolos do seu .so privado mapeados em build.rs
#[link(name = "infinite_context_engine", kind = "dylib")]
extern "C" {
    fn prefetch_page(page_id: u64, score: f32) -> i32;
    fn insert_page_to_ram(page_id: u64, tokens_ptr: *const u32, len: usize) -> i32;
}

const TOKENS_PER_PAGE: usize = 1024;

fn bench_engine_pipeline(c: &mut Criterion) {
    // 1. Inicializa o interpretador Python em background para binários standalone/benches
    pyo3::prepare_freethreaded_python();

    // 2. Garante o bloqueio do GIL para subir as tabelas globais de símbolos
    pyo3::Python::with_gil(|_py| {
        // Inicializado com sucesso dentro do escopo do executável nativo
    });

    let _runtime = tokio::runtime::Runtime::new().unwrap();
    let swap_file = "benchmark_swap_file.bin";
    let _ = fs::remove_file(swap_file);

    // Aloca as páginas iniciais simuladas injetando direto no seu .so privado
    for i in 0..20 {
        let mock_tokens = vec![i as u32; TOKENS_PER_PAGE];
        unsafe {
            insert_page_to_ram(i, mock_tokens.as_ptr(), mock_tokens.len());
        }
    }

    c.bench_function("prefetch_page_with_eviction", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                unsafe {
                    prefetch_page(1, 0.95);
                }
            }
            start.elapsed()
        })
    });
}

criterion_group!(benches, bench_engine_pipeline);
criterion_main!(benches);