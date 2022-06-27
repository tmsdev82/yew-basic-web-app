use web_sys::{HtmlInputElement};
use yew::{prelude::*};

#[function_component(ListComponent)]
pub fn list_component() -> Html {
    let name_list = use_state(Vec::new);
    let input_ref: NodeRef = NodeRef::default();

    let on_click = {
        let name_list = name_list.clone();
        let cur_input = input_ref.clone();
        Callback::from(move |_e: MouseEvent| {
            let mut names = (*name_list).clone();
            let name_input_element = cur_input.cast::<HtmlInputElement>().unwrap(); 
            let new_name = name_input_element.value();
            name_input_element.set_value("");
            names.push(new_name);
            name_list.set(names);
            
        })
    };

    let display_names = (*name_list).iter().map(|name| html!{<li>{name}</li>});
    html!{
        <div>
            <div>
            {"List component"}
            </div>
            <div>
                <input ref={input_ref} type="text" placeholder="input a name"/>
                <button onclick={on_click}>{"Add to list"}</button>
            </div>
            <div>
                <ul>
                {for display_names}              
                </ul>
            </div>
        </div>
    }
}