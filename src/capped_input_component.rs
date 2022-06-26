use web_sys::{HtmlInputElement};
use yew::{Component, html, Html, 
    Properties, InputEvent, TargetCast, NodeRef, MouseEvent};

pub enum Msg {
    SetInput(u32),
    AddValue(u32),
    DoNothing
}

#[derive(Properties, PartialEq)]
pub struct CappedInputProps {
    pub min_value: u32,
    pub max_value: u32
}

pub struct CappedInputComponent {
    input_ref: NodeRef,
    value_list: Vec<u32>
}

impl Component for CappedInputComponent {
    type Message = Msg;
    type Properties = CappedInputProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self  { input_ref:NodeRef::default(), value_list: Vec::new() }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInput(val) => {
                log::debug!("Setting input value to: {}", val);
                let input_element = self.input_ref.clone().cast::<HtmlInputElement>().unwrap();
                input_element.set_value(&val.to_string());
                true
            },
            Msg::AddValue(val) => {
                log::debug!("Adding value to list: {}", val);
                self.value_list.push(val);
                true
            },
            Msg::DoNothing => {
                log::debug!("Do nothing");
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {

        let min_val = ctx.props().min_value;
        let max_val = ctx.props().max_value;

        let on_click = {            
            let cur_input = self.input_ref.clone();
            ctx.link().callback(move |_e: MouseEvent| {
                let value_input_element = cur_input.cast::<HtmlInputElement>().unwrap(); 
                let new_value = value_input_element.value();
                value_input_element.set_value("");
                match new_value.parse() {
                    Ok(val) => Msg::AddValue(val),
                    Err(_) => {
                        log::debug!("Error occured parsing '{}'", new_value);
                        Msg::DoNothing
                    }
                }
            })
        };

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

        let display_values = self.value_list.iter().map(|value| html!{<li>{value}</li>});
        html! {
            <div>
                <div>
                    <label>{format!("Input value. Min: {}, Max: {}", min_val, max_val)}</label>
                    <input ref={self.input_ref.clone()} type="number" placeholder="input a number" oninput={on_input} min={min_val.to_string()} max={max_val.to_string()}/>              
                    <button onclick={on_click}>{"Add to list"}</button>
                </div>
                <div>
                    <ul>
                    {for display_values}
                
                    </ul>
                </div>
            </div>
        }
    }
}