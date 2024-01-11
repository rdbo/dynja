use dynja::Template;

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct MyTemplate<'a> {
    name: &'a str,
}

// Current way of setting filters (not seamless unfortunately)
#[cfg(not(debug_assertions))]
pub mod filters {
    pub fn myfilter(_input: &str) -> Result<String, askama::Error> {
        Ok("Filter Applied (askama)".into())
    }
}

#[cfg(debug_assertions)]
fn setup_filters() {
    let env = dynja::environment();
    env.lock()
        .unwrap()
        .add_filter("myfilter", |_input: String| "Filter Applied (minijinja)");
}

#[cfg(not(debug_assertions))]
fn setup_filters() {}

fn main() {
    println!("dynja tests");

    setup_filters();

    let template = MyTemplate { name: "Test" };
    println!("Template: {:?}", template);
    println!("Template Render: {}", template.render().unwrap());
}
