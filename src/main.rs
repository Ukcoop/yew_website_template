use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="p-5 flex">
            <button class="bg-slate-700 py-2 px-4 rounded-md" {onclick}>{ "+1" }</button>
            <p class="bg-slate-800 ml-2 py-2 px-4 text-lg rounded-md">{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
