use handlebars::Handlebars;
use serde_json::Value;
use std::sync::OnceLock;

// Global registry
static REGS: OnceLock<Handlebars> = OnceLock::new();

pub fn setup_hbs() -> &'static Handlebars<'static> {
    REGS.get_or_init(|| {
        let mut registry = Handlebars::new();
        // You can pre-register your templates here
        registry
            .register_template_string(
                "brands",
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/resources/brands_request.json.hbs"
                )),
            )
            .unwrap();
        registry
            .register_template_string(
                "railways",
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/resources/railways_request.json.hbs"
                )),
            )
            .unwrap();
        registry
            .register_template_string(
                "scales",
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/resources/scales_request.json.hbs"
                )),
            )
            .unwrap();
        registry
            .register_template_string(
                "catalog_items",
                include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resources/catalog_items_request.json.hbs"
                )),
            )
            .unwrap();
        registry
    })
}

pub fn render(registry: &'static Handlebars<'static>, template_name: &str, data: Value) -> Value {
    let rendered = registry
        .render(template_name, &data)
        .expect("Failed to render template.");
    serde_json::from_str(&rendered).expect("Failed to deserialize template.")
}
