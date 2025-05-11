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
    let urgency = use_state(|| 0);
    let cleaner_speed = use_state(|| 0);
    let booked_long = use_state(|| false);

    let tasks = use_state(|| vec![] as Vec<Task>);
    let room_list = use_state(|| vec![] as Vec<Room>);

    let on_submit = {
        let room_list_state = room_list.clone();
        let tasks_state = tasks.clone();

        let room = Room {
            room_no: *room_no,
            is_clean: *is_clean,
            is_occupied: *is_occupied,
            urgency: *urgency,
            cleaner_speed: *cleaner_speed,
            booked_long: *booked_long,
        };

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let mut updated_rooms = (*room_list_state).clone();
            updated_rooms.push(room.clone());
            room_list_state.set(updated_rooms.clone());

            let tasks_state = tasks_state.clone();
            spawn_local(async move {
                let Ok(resp) = Request::post("http://localhost:3000/schedule")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&updated_rooms).unwrap())
                    .send()
                    .await
                else {
                    return;
                };

                let Ok(data) = resp.json::<Vec<Task>>().await else {
                    return;
                };

                tasks_state.set(data);
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

                <div>
                    <label for="urgency">{ "Urgency of Cleaning:" }</label>
                    <input
                        type="range"
                        id="urgency"
                        min="0"
                        max="10"
                        value={urgency.to_string()}
                        style={format!("background: linear-gradient(to right, green, red {}%)", *urgency * 10)}
                        oninput={Callback::from({
                            let urgency = urgency.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    urgency.set(input.value_as_number() as u8);
                                }
                            }
                        })}
                    />
                </div>

                <div>
                    <label for="speed">{ "Cleaner Speed Required:" }</label>
                    <input
                        type="range"
                        id="speed"
                        min="0"
                        max="10"
                        value={cleaner_speed.to_string()}
                        style={format!("background: linear-gradient(to right, green, red {}%)", *cleaner_speed * 10)}
                        oninput={Callback::from({
                            let cleaner_speed = cleaner_speed.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    cleaner_speed.set(input.value_as_number() as u8);
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
                                let booked_long = booked_long.clone();
                                move |e: Event| {
                                    let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
                                    booked_long.set(input.checked());
                                }
                            })}
                        />
                        { "Booked for more than 3 days?" }
                    </label>
                </div>

                <button type="submit">{ "Add Room & Recalculate" }</button>
            </form>

            <h2>{ "Cleaning Priority" }</h2>
            <ul>
                { for tasks.iter().map(|t| html! {
                    <li>{ format!("Room {} - Priority {}", t.room_no, t.priority) }</li>
                }) }
            </ul>
        </div>
    }
}
