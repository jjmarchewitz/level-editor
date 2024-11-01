//! Definitions for the 9x9 grid that makes up the playable area
//! bottom left is (0, 0)
//! top right is (8, 8)

use leptos::*;
use wasm_bindgen::JsValue;
use web_sys;

const GRID_SIZE: u32 = 9;

#[component]
pub fn Grid() -> impl IntoView {
    // Generate the 9x9 grid
    view! {
        <div style="display: grid; grid-template-columns: repeat(9, 50px); grid-template-rows: repeat(9, 50px); gap: 0px;">
            { (0..GRID_SIZE).flat_map(|row| {
                (0..GRID_SIZE).map(move |col| view! {
                    <Cell row=row col=col/>
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>() }
        </div>
    }
}

#[component]
pub fn Cell(row: u32, col: u32) -> impl IntoView {
    let row = GRID_SIZE - row - 1;
    // This function is called when a square is clicked
    let on_square_click = move |row: u32, col: u32| {
        // Replace with the action you want to perform on click
        // println!("Square clicked at ({}, {})", row, col);
        web_sys::console::log_1(&JsValue::from(format!(
            "Square clicked at ({}, {})",
            row, col
        )));
    };

    view! {
        <div
            style="width: 50px; height: 50px; background-color: lightgray; border: 1px solid black; display: flex; align-items: center; justify-content: center; cursor: pointer;"
            on:click=move |_| on_square_click(row, col)
        >
            {format!("{}-{}", row, col)}
        </div>
    }
}
