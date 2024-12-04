#![forbid(unsafe_code)]

use leptos::prelude::*;
use std::str::FromStr;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[derive(Debug, Clone, PartialEq, strum::EnumString, strum::Display)]
enum Mode {
    //
    // // TODO: Investigate index out of bounds
    //
    // Chars,
    //
    // // TODO: Make unicode work
    //
    // Graphemes,
    Lines,
    //
    // // TODO: Make unicode work
    //
    // UnicodeWords,
    //
    // // TODO: Investigate index out of bounds
    //
    // Words,
}

#[component]
fn App() -> impl IntoView {
    // Input
    let (old_text, set_old_text) = signal(String::new());
    let (new_text, set_new_text) = signal(String::new());
    //
    // // TODO: Show text lines
    // // TRY: HTML editor instead of textarea elements
    // // - https://codemirror.net/
    // // - https://ace.c9.io/
    // let text_lines = move || {
    //     let old_text_lines = old_text.get().lines().count();
    //     let new_text_lines = new_text.get().lines().count();
    //     let lines_num = old_text_lines.max(new_text_lines);
    //     (1..=lines_num)
    //         .map(|i| format!("{i}\n"))
    //         .collect::<String>()
    // };
    //
    // Options
    let (range, set_range) = signal(3);
    let (mode, set_mode) = signal(Mode::Lines);
    // Output
    let output = view! {
        {move || {
            let old = old_text.get();
            let new = new_text.get();
            // Computing
            let diff = match mode.get() {
                //
                // // TODO: Investigate index out of bounds
                //
                // Mode::Chars => similar::TextDiff::from_chars(&old, &new),
                //
                // // TODO: Make unicode work
                //
                // Mode::Graphemes => similar::TextDiff::from_graphemes(&old, &new),
                Mode::Lines => similar::TextDiff::from_lines(&old, &new),
                //
                // // TODO: Make unicode work
                //
                // Mode::UnicodeWords => similar::TextDiff::from_unicode_words(&old, &new),
                //
                // // TODO: Investigate index out of bounds
                //
                // Mode::Words => similar::TextDiff::from_words(&old, &new),
            };
            let operations = diff.ops();
            // Grouping
            let groupped_operations = similar::group_diff_ops(Vec::from(operations), range.get());
            groupped_operations
                .into_iter()
                .map(|group| {
                    view! {
                        <table>
                            <tbody>
                                {group
                                    .into_iter()
                                    .map(|operation| {
                                        let old = old_text.get();
                                        let new = new_text.get();
                                        let diff = similar::TextDiff::from_lines(&old, &new);
                                        //
                                        // // TODO: Inline changes (requires implementing time for wasm, or using the `instant` crate)
                                        // diff.iter_inline_changes(&operation)
                                        //
                                        diff.iter_changes(&operation)
                                            .map(|change| {
                                                let old_index = change.old_index();
                                                let new_index = change.new_index();
                                                let (sign, color) = match change.tag() {
                                                    similar::ChangeTag::Equal => ("=", "gray"),
                                                    similar::ChangeTag::Delete => ("-", "red"),
                                                    similar::ChangeTag::Insert => ("+", "green"),
                                                };
                                                view! {
                                                    <tr>
                                                        <td style=format!(
                                                            "color:{color}",
                                                        )>
                                                            {old_index.map(|n| n + 1)}
                                                        </td>
                                                        <td style=format!(
                                                                "color:{color}",
                                                        )>
                                                            {new_index.map(|n| n + 1)}
                                                        </td>
                                                        <td style=format!(
                                                                "color:{color}",
                                                        )>
                                                            {sign}
                                                        </td>
                                                        <td style=format!(
                                                                "color:{color}",
                                                        )>
                                                            {format!(
                                                                "{}{}",
                                                                change.value(),
                                                                if change.missing_newline() { "\n" } else { "" },
                                                            )}
                                                        </td>
                                                    </tr>
                                                }
                                            })
                                            .collect::<Vec<_>>()
                                    })
                                    .collect::<Vec<_>>()}
                            </tbody>
                        </table>
                        <hr />
                    }
                })
                .collect::<Vec<_>>()
        }}
    };

    view! {
        <h1>"Diffing text"</h1>
        <div>
            <h2>"Input"</h2>
            <div class="flex-container" style="display:flex; flex-wrap: wrap;">
                //
                // // TODO: Show line numbers in the input
                // <textarea
                //     style="flex: 1;"
                //     placeholder="1"
                //     disabled
                //     // the `prop:` syntax lets you update a DOM property,
                //     // rather than an attribute.
                //     prop:value=text_lines
                // >
                //     {move || text_lines()}
                // </textarea>
                //
                <textarea
                    autofocus
                    rows="10"
                    style="flex: 1000;"
                    placeholder="Old text."
                    on:input=move |ev| {
                        set_old_text.set(event_target_value(&ev));
                    }
                    prop:value=old_text
                >
                    {move || old_text.get()}
                </textarea>
                <textarea
                    rows="10"
                    style="flex: 1000;"
                    placeholder="New text."
                    on:input=move |ev| {
                        set_new_text.set(event_target_value(&ev));
                    }
                    prop:value=new_text
                >
                    {move || new_text.get()}
                </textarea>
            </div>
        </div>
        <div>
            <h2>"Options"</h2>
            <div>

                // TODO: Select the mode of diffing
                <label for="mode">"Mode:"</label>
                <select
                    on:change:target=move |ev| {
                        set_mode.set(Mode::from_str(&ev.target().value()).unwrap());
                    }
                    prop:value=move || mode.get().to_string()
                >
                    <option value="Lines">"Lines"</option>
                    // <option value="Chars">"Characters"</option>
                    // <option value="Graphemes">"Graphemes"</option>
                    // <option value="UnicodeWords">"Unicode words"</option>
                    // <option value="Words">"Words"</option>
                </select>

                <label for="context">"Context:"</label>
                <input
                    type="number"
                    name="context"
                    on:input:target=move |ev| {
                        set_range.set(ev.target().value().parse().unwrap_or(3));
                    }
                    prop:value=range
                />
            </div>
        </div>
        <div>
            <h2>"Output"</h2>
            <div>{output}</div>
        </div>

        // Description
        <footer>
            <h2>"About"</h2>

            <h3>"Diffing implementation"</h3>
            <p>
                <a href="https://github.com/mitsuhiko/similar">"Similar"</a>
                " is a dependency free crate for Rust
                that implements different diffing algorithms and high level interfaces for it.
                It is based on the "
                <a href="https://pijul.org/">"pijul"</a>
                " implementation of the Patience algorithm and
                inherits some ideas from there.
                It also incorporates the Myer's diff algorithm which was largely written by Brandon Williams.
                Similar was built for the "
                <a href="https://insta.rs">"insta snapshot testing library"</a>
                .
            </p>

            <h3>"This playground"</h3>
            <p>
                "Source code: "
                <a href="https://github.com/saona-raimundo/similar_playground">"GitHub"</a>
            </p>
            <p>
                "Lisence: "
                <a rel="lisence" href="https://creativecommons.org/publicdomain/zero/1.0/">
                    <img
                        alt="Creative Commons Licence"
                        style="border-width:0"
                        src="https://i.creativecommons.org/l/by/4.0/80x15.png"
                    />
                </a> <a rel="lisence" href="https://creativecommons.org/publicdomain/zero/1.0/">
                    {"CC0 1.0 Universal"}
                </a>
            </p>
            <address>
                {"Author: 🧑🏼‍💻"}
                <a href="href=https://saona-raimundo.github.io/">{"Raimundo Saona"}</a>
            </address>
        </footer>
    }
}
