use crate::components::counter_btn::Button;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <div class="buttons">
                    <Button/>
                    <Button increment=5/>
                </div>

                <div class="container gap-6 grid grid-cols-1 pt-20 mx-auto text-center">

                    <h1 class="text-6xl tracking-wide">"Welcome to Leptos"</h1>

                    <div class="mx-auto">
                        <label class="flex cursor-pointer gap-2">
                            <span class="label-text">Current</span>
                            <input type="checkbox" value="cupcake" class="toggle theme-controller"/>
                            <span class="label-text">Cupckake</span>
                        </label>
                    </div>

                    <div>
                        <button class="btn btn-primary my-4" on:click=move |_| *count.write() += 1>
                            Click me
                            {count}
                        </button>

                    </div>
                    <div>
                        <button class="btn" onclick="my_modal_1.showModal()">
                            open modal
                        </button>
                        <dialog id="my_modal_1" class="modal">
                            <div class="modal-box">
                                <h3 class="text-lg font-bold">Hello!</h3>
                                <p class="py-4">Press ESC key or click the button below to close</p>
                                <div class="modal-action">
                                    <form method="dialog">
                                        <button class="btn">Close</button>
                                    </form>
                                </div>
                            </div>
                        </dialog>
                    </div>
                </div>

            </div>
        </ErrorBoundary>
    }
}
