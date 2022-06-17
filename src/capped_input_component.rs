use web_sys::{HtmlInputElement};
use yew::{Component, html, Html, Properties, InputEvent, TargetCast, NodeRef};

pub enum Msg {
    SetInput(u32)
}

#[derive(Properties, PartialEq)]
pub struct CappedInputProps {
    pub min_value: u32,
    pub max_value: u32
}

pub struct CappedInputComponent {
    input_ref: NodeRef
}

impl Component for CappedInputComponent {
    type Message = Msg;
    type Properties = CappedInputProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self  { input_ref:NodeRef::default() }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInput(val) => {
                log::debug!("Setting input value to: {}", val);
                let input_element = self.input_ref.clone().cast::<HtmlInputElement>().unwrap();
                input_element.set_value(&val.to_string());
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {

        let min_val = ctx.props().min_value;
        let max_val = ctx.props().max_value;

        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input_el: HtmlInputElement = e.target_unchecked_into();

            let mut val: u32 = match input_el.value().parse() {
                Ok(val) => val,
                Err(err) => {
                    log::error!("error ocurred parsing value: {}", err);
                    0
                }
            };

            log::debug!("Input value: {}", val);
            if val > max_val {
                val = max_val;
            } else if val < min_val {
                val = min_val
            }

            Msg::SetInput(val)
        });
        
        html! {
            <div>
                <label>{format!("Input value. Min: {}, Max: {}", min_val, max_val)}</label>
                <input ref={self.input_ref.clone()} type="number" placeholder="input a number" oninput={on_input} min={min_val.to_string()} max={max_val.to_string()}/>              
            </div>
        }
    }
}