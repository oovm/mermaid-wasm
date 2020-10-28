pub use mermaid_wasmbind::{MermaidOptions};
use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};
use yew::utils::document;

#[derive(Properties, Clone, PartialEq)]
pub struct MermaidProperties {
    pub code: String,
    #[prop_or(true)]
    pub arrow_marker_absolute: bool,
    #[prop_or(String::from("default"))]
    pub theme: String,
}

pub struct Mermaid {
    pub props: MermaidProperties,
}

impl Component for Mermaid {
    type Message = ();
    type Properties = MermaidProperties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props == props {
            true => false,
            false => {
                self.props = props;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let drawer = self.create_drawer();
        let t = yew::utils::document().create_element("div").unwrap();
        t.set_inner_html(&drawer.render(&t, &self.props.code));
        Html::VRef(t.into())
    }
    #[cfg(feature = "auto-cdn")]
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.load_cdn().unwrap_or_default()
        }
    }
}

impl Mermaid {
    pub fn create_drawer(&self) -> MermaidOptions {
        let mut render = MermaidOptions::default();
        render.set_theme(&self.props.theme);
        return render;
    }
    pub fn load_cdn(&self) -> Result<(), std::io::Error> {
        // <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.12.0/dist/katex.min.css">
        if let None = document().get_element_by_id("cdn-katex") {
            let head = document().query_selector("head").expect("").expect("");
            let t = document().create_element("link").expect("");
            // async css load
            t.set_attribute("id", "cdn-katex").expect("");
            t.set_attribute("media", "none").expect("");
            t.set_attribute("onload", "this.media='all'").expect("");
            t.set_attribute("rel", "stylesheet").expect("");
            t.set_attribute("href", "https://cdn.jsdelivr.net/npm/katex@0.12.0/dist/katex.min.css").expect("");
            head.append_child(&t).expect("");
        }
        Ok(())
    }
}
