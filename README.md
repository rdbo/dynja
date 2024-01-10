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

## License
This project is licensed under the `GNU AGPL-3.0`. No later versions allowed.

Read the `LICENSE` file in the root directory of the project for more information.

## Considerations
Even though MiniJinja and Askama are both related to Jinja, they are not 100% compatible with each other. So be wary of inconsistencies!
