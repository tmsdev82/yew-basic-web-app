use yew::prelude::*;
use capped_input_component::CappedInputComponent;

mod capped_input_component;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div class="container">
            <h1>{"Basic Yew Web App"}</h1>
            <div>
                <CappedInputComponent min_value={0} max_value={20}/>  
                <CappedInputComponent min_value={5} max_value={30}/>  
            </div>
        </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
