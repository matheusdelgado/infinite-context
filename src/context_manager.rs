use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::time::Instant;

// Assinaturas da ABI C vindas do seu .so privado
#[link(name = "infinite_context_engine", kind = "dylib")]
extern "C" {
    fn prefetch_page(page_id: u64, score: f32) -> i32;
    fn insert_page_to_ram(page_id: u64, tokens_ptr: *const u32, len: usize) -> i32;
}

const TOKENS_PER_PAGE: usize = 1024;

fn bench_engine_pipeline(c: &mut Criterion) {
    // Inicializa o GIL e as tabelas globais de símbolos (PyExc_ValueError, etc.)
    pyo3::Python::with_gil(|_py| {
        // Inicializado com sucesso no contexto do executável
    });

    let _runtime = tokio::runtime::Runtime::new().unwrap();
    let swap_file = "benchmark_swap_file.bin";
    let _ = fs::remove_file(swap_file);

    // Aloca as páginas iniciais simuladas
    for i in 0..20 {
        let mock_tokens = vec![i as u32; TOKENS_PER_PAGE];
        unsafe {
            insert_page_to_ram(i, mock_tokens.as_ptr(), mock_tokens.len());
        }
    }

    c.bench_function("prefetch_page_with_eviction", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for i in 0..iters {
                let target_page = (i % 20) as u64;
                unsafe {
                    prefetch_page(target_page, 0.9);
                }
            }
            start.elapsed()
        });
    });

    let _ = fs::remove_file(swap_file);
}

criterion_group!(benches, bench_engine_pipeline);
criterion_main!(benches);