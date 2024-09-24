use inquire::{
    ui::{Color, RenderConfig, StyleSheet, Styled},
    Text,
};

use crate::{error::AppError, types::Verb};

pub fn run_verb_prompt(verbs: Vec<Verb>) -> Result<(), AppError> {
    let render_config = RenderConfig::default()
        .with_help_message(StyleSheet::default().with_fg(Color::rgb(136, 57, 240)))
        .with_answer(StyleSheet::default().with_fg(Color::rgb(76, 79, 106)))
        .with_answered_prompt_prefix(Styled::new(""));

    for verb in verbs {
        let expansion = verb
            .en_expanded
            .map(|s| format!("({})", s))
            .unwrap_or("".to_string());
        println!("{:>18} {}", verb.en, expansion);

        // Infinitiv
        Text::new("")
            .with_help_message("Infinitiv")
            .with_formatter(&|i| {
                if i == verb.de {
                    format!("{:>16} │ {:16} │ ✅", "Infinitiv", i)
                } else {
                    format!("{:>16} │ {:16} │ ❌ {}", "Infinitiv", i, verb.de)
                }
            })
            .with_render_config(render_config)
            .prompt()?;

        // Conjugation #1
        Text::new("")
            .with_help_message("Präsens")
            .with_formatter(&|i| {
                let correct = verb.de_forms.get(1).unwrap();
                if i == correct {
                    format!("{:>16} │ {:16} │ ✅", "Präsens", i)
                } else {
                    format!("{:>16} │ {:16} │ ❌ {}", "Präsens", i, correct)
                }
            })
            .with_render_config(render_config)
            .prompt()?;

        // Conjugation #2
        Text::new("")
            .with_help_message("Präteritum")
            .with_formatter(&|i| {
                let correct = verb.de_forms.get(2).unwrap();
                if i == correct {
                    format!("{:>16} │ {:16} │ ✅", "Präteritum", i)
                } else {
                    format!("{:>16} │ {:16} │ ❌ {}", "Präteritum", i, correct)
                }
            })
            .with_render_config(render_config)
            .prompt()?;

        // Conjugation #3
        Text::new("")
            .with_help_message("Perfekt")
            .with_formatter(&|i| {
                let correct = verb.de_forms.get(3).unwrap();
                if i == correct {
                    format!("{:>16} │ {:16} │ ✅", "Perfekt", i)
                } else {
                    format!("{:>16} │ {:16} │ ❌ {}", "Perfekt", i, correct)
                }
            })
            .with_render_config(render_config)
            .prompt()?;

        println!("");
    }

    Ok(())
}
