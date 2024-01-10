use dynja::Template;

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct MyTemplate<'a> {
    name: &'a str,
}

fn main() {
    println!("dynja tests");

    let template = MyTemplate { name: "Test" };
    println!("Template: {:?}", template);
    println!("Template Render: {}", template.render().unwrap());
}
