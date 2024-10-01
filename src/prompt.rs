use inquire::{
    ui::{Color, RenderConfig, StyleSheet, Styled},
    Text,
};

use crate::{error::AppError, types::Verb};

fn split_raw_word<'a>(word: &'a str, sep: &'static str) -> (&'a str, Option<&'a str>) {
    let mut split = word.split(sep);
    (split.next().unwrap_or(word), split.next())
}

fn is_correct(answer: &str, correct_raw: &String) -> bool {
    if correct_raw.contains(" ") {
        let (aux, body) = split_raw_word(correct_raw, " ");
        let split_aux = split_raw_word(aux, "/");
        let split_body = split_raw_word(body.unwrap(), "/");
        let (opt1, opt2) = match (split_aux, split_body) {
            ((a1, Some(a2)), (b, None)) => (format!("{a1} {b}"), format!("{a2} {b}")),
            ((a, None), (b1, Some(b2))) => (format!("{a} {b1}"), format!("{a} {b2}")),
            _ => (correct_raw.clone(), correct_raw.clone()),
        };

        answer == opt1 || answer == opt2
    } else {
        let (opt1, opt2) = split_raw_word(correct_raw, "/");

        answer == opt1 || opt2.is_some_and(|opt| answer == opt)
    }
}

fn format_answered(answer: &str, correct: &String, tense: &str) -> String {
    if is_correct(answer, correct) {
        format!("{:>16} │ {:16} │ ✅ {}", tense, answer, correct)
    } else {
        format!("{:>16} │ {:16} │ ❌ {}", tense, answer, correct)
    }
}

pub fn run_verb_prompt(verbs: Vec<Verb>) -> Result<(), AppError> {
    let mut correct = 0;
    let mut total = 0;

    let render_config = RenderConfig::default()
        .with_help_message(StyleSheet::default().with_fg(Color::rgb(136, 57, 240)))
        .with_answer(StyleSheet::default().with_fg(Color::rgb(76, 79, 106)))
        .with_answered_prompt_prefix(Styled::new(""));

    for verb in verbs {
        let mut answers: Vec<bool> = vec![];

        let expansion = verb
            .en_expanded
            .map(|s| format!("({})", s))
            .unwrap_or("".to_string());
        println!("{:>18} {}", verb.en, expansion);

        // Infinitiv
        answers.push(
            Text::new("")
                .with_help_message("Infinitiv")
                .with_formatter(&|i| format_answered(i, &verb.de, "Infinitiv"))
                .with_render_config(render_config)
                .prompt()?
                == verb.de,
        );

        // Conjugation #1
        let pres = verb.de_forms.get(1).unwrap();
        answers.push(
            Text::new("")
                .with_help_message("Präsens")
                .with_formatter(&|i| format_answered(i, pres, "Präsens"))
                .with_render_config(render_config)
                .prompt()?
                == *pres,
        );

        // Conjugation #2
        let pret = verb.de_forms.get(2).unwrap();
        answers.push(
            Text::new("")
                .with_help_message("Präteritum")
                .with_formatter(&|i| format_answered(i, pret, "Präteritum"))
                .with_render_config(render_config)
                .prompt()?
                == *pret,
        );

        // Conjugation #3
        let perf = verb.de_forms.get(3).unwrap();
        answers.push(
            Text::new("")
                .with_help_message("Perfekt")
                .with_formatter(&|i| format_answered(i, perf, "Perfekt"))
                .with_render_config(render_config)
                .prompt()?
                == *perf,
        );

        total += answers.len();
        correct += answers.iter().map(|a| *a as i32).sum::<i32>();

        println!("");
    }
    println!("🇩🇪 {correct} correct out of {total}! 🇩🇪\n");

    Ok(())
}
