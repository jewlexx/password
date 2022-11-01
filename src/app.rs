use rand::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::password::Password;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Properties)]
pub struct GenConf {
    pub length: usize,
    pub count: usize,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

fn generate_password(config: GenConf) -> String {
    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &[u8] = b"0123456789";
    const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{};':,./<>?";

    let mut rng = thread_rng();

    let mut password = String::with_capacity(config.length);

    let mut charsets = Vec::new();
    if config.uppercase {
        charsets.push(UPPERCASE);
    }
    if config.lowercase {
        charsets.push(LOWERCASE);
    }
    if config.numbers {
        charsets.push(NUMBERS);
    }
    if config.symbols {
        charsets.push(SYMBOLS);
    }

    for _ in 0..config.length {
        let charset = charsets.choose(&mut rng).unwrap();
        let c = charset.choose(&mut rng).unwrap();
        password.push(*c as char);
    }

    password
}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    const DEFAULT_CONFIG: GenConf = GenConf {
        length: 16,
        count: 1,
        uppercase: true,
        lowercase: true,
        numbers: true,
        symbols: true,
    };

    // Force re-render when the following changes allowing it to be triggered by the button
    let force_gen: UseStateHandle<u8> = use_state(|| 0);

    let config = use_state_eq(|| DEFAULT_CONFIG);
    let config_raw = *config;

    let password = generate_password(*config);

    println!("{password}");

    let oninput_len = Callback::from({
        let config = config.clone();

        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            debug!("Input changed to: {}", input.value());

            let old_config = *config;

            config.set(GenConf {
                length: input.value_as_number() as usize,
                ..old_config
            });
        }
    });

    let oninput_count = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();

        debug!("Input changed to: {}", input.value());

        let old_config = *config;

        config.set(GenConf {
            count: input.value_as_number() as usize,
            ..old_config
        });
    });

    let onclick = {
        Callback::from(move |_| {
            if *force_gen > 0 {
                force_gen.set(0);
            } else {
                force_gen.set(1);
            }
        })
    };

    let passwords = (0..config_raw.count)
        .map(|_| generate_password(config_raw))
        .collect::<Vec<_>>();

    html! {
        <div>
            <h1>{ "Password Generator" }</h1>
            <div class="config">
                <h3>{ format!("Length ({})", config_raw.length) }</h3>
                <input type="range" min="1" max="128" value={config_raw.length.to_string()} oninput={oninput_len} />

                <h3>{ format!("Count ({})", config_raw.count) }</h3>
                <input type="range" min="1" max="10" value={config_raw.count.to_string()} oninput={oninput_count} />

                <button class="regenerate" {onclick}>{ "↻" }</button>
            </div>

            <Password passwords={passwords} />
        </div>
    }
}
