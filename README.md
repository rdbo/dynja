# Dynja
![dynja-logo](https://raw.githubusercontent.com/rdbo/dynja/master/LOGO.png)
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
dynja = { version = "0.4", features = ["askama_release"] }
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

NOTE: You can use `dynja` without askama if you wish. Just add the following in your `Cargo.toml` instead:
```toml
dynja = "0.4"
```
It has optimizations for `minijinja` on release mode as well, but it won't be as performant as `askama`.

Read the [Considerations](#Considerations) section for more information.

Have fun!

## Benchmarks
NOTE: These benchmarks are not done properly, so they don't represent a real world scenario. They do let you see the difference between switching the engines though.

Tested on `dynja_bench`, located on the root directory of this repository.

Dynja 0.4.0 (Debug)
```
Benchmarking: MiniJinja
<!DOCTYPE html>
<html>
  <head></head>

  <body>
    <h1>Dynja Benchmark</h1>
    <h2>Name: Tests</h2>
    <h2>Number: 1337</h2>
    <h2>Float: 420.0</h2>
  </body>
</html>

Iteration: 999999
Benchmark finished
Time taken to finish iterations: 103629ms (103s)
```

Dynja 0.4.0 (Release - `features = ["askama_release"]`)
```
Benchmarking: Askama
<!DOCTYPE html>
<html>
  <head></head>

  <body>
    <h1>Dynja Benchmark</h1>
    <h2>Name: Tests</h2>
    <h2>Number: 1337</h2>
    <h2>Float: 420</h2>
  </body>
</html>

Iteration: 999999
Benchmark finished
Time taken to finish iterations: 937ms (0s)
```

The release build finished the iterations about 110 times faster than the debug build.

On a side note, this benchmark also doesn't say that minijinja is slow by any means.
In other to achieve hot reloading of the templates, we have to clear the cached templates of minijinja for every `render()`, which means we add a severe
bottleneck to its performance to get a better development experience. Here are the results of a test done without the `"askama_release"` feature, on release mode:

Dynja 0.4.0 (Release)
```
Benchmarking: MiniJinja
<!DOCTYPE html>
<html>
  <head></head>

  <body>
    <h1>Dynja Benchmark</h1>
    <h2>Name: Tests</h2>
    <h2>Number: 1337</h2>
    <h2>Float: 420.0</h2>
  </body>
</html>

Iteration: 999999
Benchmark finished
Time taken to finish iterations: 2851ms (2s)
```

## License
This project is licensed under the `GNU AGPL-3.0`. No later versions allowed.

Read the `LICENSE` file in the root directory of the project for more information.

## Considerations
Even though MiniJinja and Askama are both related to Jinja, they are not 100% compatible with each other. So be wary of inconsistencies!

For most cases, the slower performance of MiniJinja won't affect you as much as you think. In the benchmark above (release mode), it still manage to render the template 350,754 times in a single second, which is more than enough for, say, a web server. If you really need that performance edge and you know that your templates are compatible across the engines, Askama still takes the win with 1,067,235 renders per second, according to the benchmark. If you need to make sure that the debug templates are compatible with release templates, stick with MiniJinja.
