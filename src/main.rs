use yew::prelude::*;
use capped_input_component::CappedInputComponent;
use list_component::ListComponent;

mod capped_input_component; 
mod list_component;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div class="container">
            <h1>{"Basic Yew Web App"}</h1>
            <div>
                <CappedInputComponent min_value={0} max_value={20}/>  
                <CappedInputComponent min_value={5} max_value={30}/>  
                <ListComponent/>
            </div>
        </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting");
    yew::start_app::<App>();
}
