use std::time::Duration;

use chrono::{prelude::*, TimeDelta};
use itertools::Itertools;
use leptos::prelude::*;

enum Row {
    Period { start: Duration },
    Break { start: Duration },
}

/// Default Home Page
#[component]
pub fn Schedule() -> impl IntoView {
    let count = RwSignal::new(0);
    let NUM_PERIODS = 11;
    let PERIOD_LENGTHS = 50; // minutes
    let BREAK_LENGTHS = 5; // minutes
    view! {
        <table class="table-auto aspect-3/2">
            <thead>
                <tr>
                    <th>period</th>
                    <th>Sun</th>
                    <th>Mon</th>
                    <th>Tue</th>
                    <th>Wed</th>
                    <th>Thu</th>
                    <th>Fri</th>
                    <th>Sat</th>
                </tr>
            </thead>
            <tbody>

                {(0..NUM_PERIODS)
                    .into_iter()
                    .map(|n| {
                        let now = Duration::from_hours(8)
                            + Duration::from_mins(n * (PERIOD_LENGTHS + BREAK_LENGTHS));
                        let mins = now.as_secs_f32() / 60.;
                        let hrs = (mins / 60.).floor();
                        let mins2 = mins - (hrs * 60.);
                        view! {
                            <tr class="h-16 border">
                                <td class="w-min text-center">
                                    <p class="text-sm">{format!("{hrs:02}:{mins2:02}")}</p>
                                    <p class="font-bold text-lg">{n + 1}</p>
                                </td>

                                {(0..7)
                                    .map(|n| view! { <td class="w-30 border">{n}</td> })
                                    .collect_view()}

                            </tr>

                            {if n != NUM_PERIODS - 1 {
                                Some(
                                    view! {
                                        <tr class="opacity-50 italic text-[.5rem] leading-none">
                                            <td>this is bull
                                                {format!("{hrs:02}:{:02}", mins2 + PERIOD_LENGTHS as f32)}
                                            </td>
                                            <td
                                                colspan="100%"
                                                class="overflow-hidden whitespace-nowrap whitespace-pre"
                                            >
                                                {"BREAK     ".repeat(35)}
                                            </td>
                                        </tr>
                                    },
                                )
                            } else {
                                None
                            }}
                        }
                    })
                    .collect_view()}

            </tbody>
        </table>
    }
}
