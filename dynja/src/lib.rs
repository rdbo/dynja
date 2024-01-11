// Re-export askama template, if askama_release is enabled and the build type is release
#[cfg(all(feature = "askama_release", not(debug_assertions)))]
pub use askama::Template;

// Declare minijinja wrapper on debug, or if askama_release isn't enabled
#[cfg(any(debug_assertions, not(feature = "askama_release")))]
mod dynja_minijinja {
    use minijinja::Environment;
    use std::sync::{Mutex, OnceLock};

    pub use dynja_derive::Template;

    pub use minijinja;

    pub trait TemplateFile {
        const PATH: &'static str; // NOTE: this is not the actual path of the template.
                                  // It's just the name of the variable taken in the #[template] macro,
                                  // just like with 'askama'.
    }

    pub fn environment() -> &'static Mutex<Environment<'static>> {
        static ENV: OnceLock<Mutex<Environment>> = OnceLock::new();
        ENV.get_or_init(|| {
            let mut env = Environment::new();
            env.set_loader(minijinja::path_loader("templates"));

            let mutex = Mutex::new(env);
            mutex
        })
    }
}

// Re-export minijinja wrapper
#[cfg(any(debug_assertions, not(feature = "askama_release")))]
pub use dynja_minijinja::*;
