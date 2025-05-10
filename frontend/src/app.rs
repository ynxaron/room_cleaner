use crate::models::{Room, Task};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let room_no = use_state(|| 0);
    let is_clean = use_state(|| false);
    let is_occupied = use_state(|| false);
    let tasks = use_state(|| vec![] as Vec<Task>);
    let room_list = use_state(|| vec![] as Vec<Room>);

    let on_submit = {
        let room_no_val = *room_no;
        let is_clean_val = *is_clean;
        let is_occupied_val = *is_occupied;

        let room_list_clone = (*room_list).clone();
        let tasks_clone = tasks.clone();
        let room_list_state = room_list.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // Prevent form submission

            let mut updated = room_list_clone.clone();
            updated.push(Room {
                room_no: room_no_val,
                is_clean: is_clean_val,
                is_occupied: is_occupied_val,
            });

            room_list_state.set(updated.clone());

            let tasks_clone2 = tasks_clone.clone();
            spawn_local(async move {
                let Ok(resp) = Request::post("http://localhost:3000/schedule")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&updated).unwrap())
                    .send()
                    .await
                else {
                    return;
                };

                let Ok(data) = resp.json::<Vec<Task>>().await else {
                    return;
                };

                tasks_clone2.set(data);
            });
        })
    };

    html! {
        <div class="container">
            <h1>{ "Room Cleaning Scheduler" }</h1>

            <form onsubmit={on_submit}>
                <div>
                    <label for="room_no">{ "Room No:" }</label>
                    <input
                        type="number"
                        id="room_no"
                        oninput={Callback::from({
                            let room_no = room_no.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    room_no.set(input.value_as_number() as u32);
                                }
                            }
                        })}
                    />
                </div>

                <div>
                    <label>
                        <input
                            type="checkbox"
                            onchange={Callback::from({
                                let is_clean = is_clean.clone();
                                move |e: Event| {
                                    let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
                                    is_clean.set(input.checked());
                                }
                            })}
                        />
                        { "Is Clean?" }
                    </label>
                </div>

                <div>
                    <label>
                        <input
                            type="checkbox"
                            onchange={Callback::from({
                                let is_occupied = is_occupied.clone();
                                move |e: Event| {
                                    let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
                                    is_occupied.set(input.checked());
                                }
                            })}
                        />
                        { "Is Occupied?" }
                    </label>
                </div>

                <button type="submit">{ "Add Room & Recalculate" }</button>
            </form>

            <h2>{ "Cleaning Priority" }</h2>
            <ul>
                { for tasks.iter().map(|t| html! {
                    <li>{ format!("Room {} - Priority {}", t.room_no, t.priority) }</li>
                })}
            </ul>
        </div>
    }
}
