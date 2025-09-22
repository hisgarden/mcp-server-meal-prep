use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::time::Instant;

/// Comprehensive Benchmark Test for Rust MCP Server
/// Tests performance, memory usage, and latency
fn benchmark_startup_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup_time");

    group.bench_function("build_time", |b| {
        b.iter(|| {
            let output = Command::new("cargo")
                .args(&["build", "--release", "--quiet"])
                .output()
                .expect("Failed to build");
            black_box(output)
        });
    });

    group.bench_function("binary_size", |b| {
        b.iter(|| {
            let output = Command::new("du")
                .args(&["-h", "target/release/mcp-server-meal-prep"])
                .output()
                .expect("Failed to get binary size");
            black_box(output)
        });
    });

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    group.bench_function("server_startup", |b| {
        b.iter(|| {
            let mut child = Command::new("target/release/mcp-server-meal-prep")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start server");

            // Send initialization request
            let request = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {
                        "name": "benchmark",
                        "version": "1.0.0"
                    }
                }
            });

            if let Some(stdin) = child.stdin.as_mut() {
                writeln!(stdin, "{}", request).unwrap();
            }

            // Wait a bit for initialization
            thread::sleep(Duration::from_millis(100));

            // Get memory usage
            let pid = child.id();
            let output = Command::new("ps")
                .args(&["-o", "rss", "-p", &pid.to_string()])
                .output()
                .expect("Failed to get memory usage");

            child.kill().unwrap();
            black_box(output)
        });
    });

    group.finish();
}

fn benchmark_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency");

    group.bench_function("list_resources", |b| {
        b.iter(|| {
            let start = Instant::now();

            let mut child = Command::new("target/release/mcp-server-meal-prep")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start server");

            let request = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "resources/list"
            });

            if let Some(stdin) = child.stdin.as_mut() {
                writeln!(stdin, "{}", request).unwrap();
            }

            let duration = start.elapsed();
            child.kill().unwrap();

            black_box(duration)
        });
    });

    group.bench_function("read_resource", |b| {
        b.iter(|| {
            let start = Instant::now();

            let mut child = Command::new("target/release/mcp-server-meal-prep")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start server");

            let request = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "resources/read",
                "params": {
                    "uri": "file://recipes/French"
                }
            });

            if let Some(stdin) = child.stdin.as_mut() {
                writeln!(stdin, "{}", request).unwrap();
            }

            let duration = start.elapsed();
            child.kill().unwrap();

            black_box(duration)
        });
    });

    group.finish();
}

fn benchmark_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");

    for concurrency in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_requests", concurrency),
            concurrency,
            |b, &concurrency| {
                b.iter(|| {
                    let mut handles = vec![];

                    for _ in 0..concurrency {
                        let handle = thread::spawn(|| {
                            let mut child = Command::new("target/release/mcp-server-meal-prep")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .stderr(Stdio::piped())
                                .spawn()
                                .expect("Failed to start server");

                            let request = json!({
                                "jsonrpc": "2.0",
                                "id": 1,
                                "method": "resources/list"
                            });

                            if let Some(stdin) = child.stdin.as_mut() {
                                writeln!(stdin, "{}", request).unwrap();
                            }

                            thread::sleep(Duration::from_millis(10));
                            child.kill().unwrap();
                        });
                        handles.push(handle);
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }

    group.finish();
}

fn benchmark_resource_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("resource_access");

    let cuisines = [
        "French",
        "Thai",
        "Italian",
        "Mexican",
        "Chinese",
        "Vietnamese",
    ];

    for cuisine in cuisines.iter() {
        group.bench_with_input(
            BenchmarkId::new("cuisine_access", cuisine),
            cuisine,
            |b, &cuisine| {
                b.iter(|| {
                    let start = Instant::now();

                    let mut child = Command::new("target/release/mcp-server-meal-prep")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to start server");

                    let request = json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "method": "resources/read",
                        "params": {
                            "uri": format!("file://recipes/{}", cuisine)
                        }
                    });

                    if let Some(stdin) = child.stdin.as_mut() {
                        writeln!(stdin, "{}", request).unwrap();
                    }

                    let duration = start.elapsed();
                    child.kill().unwrap();

                    black_box(duration)
                });
            },
        );
    }

    group.finish();
}

fn benchmark_binary_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_analysis");

    group.bench_function("binary_size", |b| {
        b.iter(|| {
            let output = Command::new("ls")
                .args(&["-lh", "target/release/mcp-server-meal-prep"])
                .output()
                .expect("Failed to get binary size");
            black_box(output)
        });
    });

    group.bench_function("dependency_count", |b| {
        b.iter(|| {
            let output = Command::new("cargo")
                .args(&["tree", "--depth", "1"])
                .output()
                .expect("Failed to get dependency tree");
            black_box(output)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_startup_time,
    benchmark_memory_usage,
    benchmark_latency,
    benchmark_throughput,
    benchmark_resource_access,
    benchmark_binary_analysis
);

criterion_main!(benches);
