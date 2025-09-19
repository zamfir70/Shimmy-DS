use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shimmy::{AppState, engine::adapter::InferenceEngineAdapter, model_registry::Registry};
use std::sync::Arc;
use std::time::Instant;

// Benchmark startup time
fn benchmark_startup(c: &mut Criterion) {
    c.bench_function("shimmy_startup", |b| {
        b.iter(|| {
            let start = Instant::now();
            let registry = Registry::default();
            let engine = Box::new(InferenceEngineAdapter::new());
            let _state = Arc::new(AppState { engine, registry });
            let duration = start.elapsed();
            black_box(duration)
        })
    });
}

// Benchmark API latency
fn benchmark_api_latency(c: &mut Criterion) {
    c.bench_function("api_latency", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        b.to_async(&rt).iter(|| async {
            let start = Instant::now();
            // Simulate minimal API request processing
            let registry = Registry::default();
            let engine = Box::new(InferenceEngineAdapter::new());
            let _state = Arc::new(AppState { engine, registry });
            let duration = start.elapsed();
            black_box(duration)
        })
    });
}

// Benchmark memory usage
fn benchmark_memory_usage(c: &mut Criterion) {
    c.bench_function("memory_baseline", |b| {
        b.iter(|| {
            let registry = Registry::default();
            let engine = Box::new(InferenceEngineAdapter::new());
            let state = Arc::new(AppState { engine, registry });
            
            // Measure memory usage here
            let memory_info = std::process::Command::new("ps")
                .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
                .output();
                
            black_box((state, memory_info))
        })
    });
}

criterion_group!(benches, benchmark_startup, benchmark_api_latency, benchmark_memory_usage);
criterion_main!(benches);
