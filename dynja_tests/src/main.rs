use dynja::*;

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct MyTemplate {
    name: &'static str,
}

fn main() {
    println!("dynja tests");

    let template = MyTemplate { name: "Hello" };
    println!("Template: {:?}", template);
    println!("Template Path: {:?}", <MyTemplate as TemplateFile>::PATH);
    println!("Template Render: {}", template.render());
}
