# Dynja
### Jinja pseudo-engine focused on DevEx and Performance

## Why Dynja?
Let's look at two of the alternatives:
- Askama: extremely fast on benchmarks, but doesn't have a very fun development experience, since you have to recompile your webserver each time you modify a template
- MiniJinja: decent performance on benchmarks, but has an awesome developer experience, with hot reloading and possibly even live reloading

So let's mix both: use MiniJinja for debug mode (better DevEx), and Askama for release mode (better performance)

And that's what Dynja essentially is

## How to use?
Add the `dynja` dependency to your `Cargo.toml`, along with the `askama` dependency. The `minijinja` dependency isn't necessary, because it is only used internally, whereas `askama` needs to be exported on release builds.
```toml
[dependencies]
dynja = "0.1"
askama = "0.12"
```
Now you can import `dynja` and use it as if it were `askama`. Nice huh?
```rust
use dynja::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct MyTemplate {
    name: &'static str,
}

fn main() {
    let template = MyTemplate { name: "Test" };
    println!("Template Render: {}", template.render().unwrap());
}
```
It will automatically choose between minijinja on debug, and askama on release, so you don't have to worry about it.

Have fun!

## Benchmarks
NOTE: These benchmarks are not done properly, so they don't represent a real world scenario. They do let you see the difference between switching the engines though.

Tested on: [https://github.com/rdbo/axum-htmx-dynja-test](https://github.com/rdbo/axum-htmx-dynja-test)

Command: `rewrk -c 100 -t 3 -h "http://127.0.0.1:8000" -d 10s`

Dynja 0.2 (Debug)
```
Beginning round 1...
Benchmarking 100 connections @ http://127.0.0.1:8000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    8.41ms   3.08ms   0.30ms   24.74ms  
  Requests:
    Total: 118376  Req/Sec: 11851.72
  Transfer:
    Total: 102.51 MB Transfer Rate: 10.26 MB/Sec
```
Dynja 0.2 (Release)
```
Beginning round 1...
Benchmarking 100 connections @ http://127.0.0.1:8000 for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max      
    2.67ms   1.10ms   0.06ms   21.09ms  
  Requests:
    Total: 373464  Req/Sec: 37380.96
  Transfer:
    Total: 323.40 MB Transfer Rate: 32.37 MB/Sec
```
The release build got over 3 times the amount of requests per second.

You may think this is due to the web server also being compiled in release vs in debug, but I've spent some time testing each individual engine on both debug and release, and these results match my previous ones.

## License
This project is licensed under the `GNU AGPL-3.0`. No later versions allowed.

Read the `LICENSE` file in the root directory of the project for more information.

## Considerations
Even though MiniJinja and Askama are both related to Jinja, they are not 100% compatible with each other. So be wary of inconsistencies!
