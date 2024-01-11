use dynja::Template;
use std::time::Instant;

#[derive(Template)]
#[template(path = "bench.html")]
struct BenchTemplate {
    name: &'static str,
    number: i64,
    float: f64,
}

fn work() {
    for i in 0..1000000 {
        let template = BenchTemplate {
            name: "Tests",
            number: 1337,
            float: 420.0,
        };

        let result = template.render().unwrap();

        if i == 0 {
            println!("{}", result);
            println!();
        }

        print!("\rIteration: {}", i);
    }

    println!();
}

fn main() {
    let engine = if cfg!(all(not(debug_assertions), feature = "askama_release")) {
        "Askama"
    } else {
        "MiniJinja"
    };
    println!("Benchmarking: {}", engine);

    let start = Instant::now();
    work();
    let stop = start.elapsed();

    println!("Benchmark finished");
    println!(
        "Time taken to finish iterations: {}ms ({}s)",
        stop.as_millis(),
        stop.as_secs()
    );
}
