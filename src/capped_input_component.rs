use yew::{Component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct CappedInputProps {
    pub min_value: u32,
    pub max_value: u32
}

pub struct CappedInputComponent {}

impl Component for CappedInputComponent {
    type Message = ();
    type Properties = CappedInputProps;
    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self  {}
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {

        let min_val = ctx.props().min_value;
        let max_val = ctx.props().max_value;
        html! {
            <div>
                <label>{format!("Input value. Min: {}, Max: {}", min_val, max_val)}</label>
                <input type="number" placeholder="input a number" min={min_val.to_string()} max={max_val.to_string()}/>              
            </div>
        }
    }
}