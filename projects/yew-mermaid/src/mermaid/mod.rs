pub use mermaid_wasmbind::MermaidOptions;
use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};

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
        Html::VRef(t.first_child().unwrap().into())
    }
}

impl Mermaid {
    pub fn create_drawer(&self) -> MermaidOptions {
        let mut render = MermaidOptions::default();
        render.set_theme(&self.props.theme);
        return render;
    }
}
