use lazy_static::lazy_static;
use ratatui::prelude::Stylize;
#[cfg(feature = "ratatui")]
use ratatui::text::Span;
#[cfg(feature = "ratatui")]
use ratatui::style::Color;
#[cfg(feature = "tuirs")]
use tui::text::Span;
#[cfg(feature = "tuirs")]
use tui::style::Color;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    pub static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
    pub static ref THEME: Theme = THEME_SET.themes["base16-ocean.dark"].clone();
}

pub(super) fn highlight_syntax(line: &str, extension: &str, start: Option<usize>, end: Option<usize>) -> Vec<Span<'static>> {
    let syntax = SYNTAX_SET.find_syntax_by_extension(extension).expect("Could not find extension");
    let mut highlight_lines = HighlightLines::new(&syntax, &THEME);
    let result = highlight_lines.highlight_line(line, &SYNTAX_SET).unwrap();

    let mut highlighted_line: Vec<Span> = vec![];

    match start {
        None => no_slice_spans(&mut highlighted_line, result),
        Some(start) => match end {
            None => start_slice_spans(&mut highlighted_line, result, start),
            Some(end) => start_end_slice_spans(&mut highlighted_line, result, start, end)
        }
    }

    return highlighted_line;
}

#[inline]
fn no_slice_spans(highlighted_line: &mut Vec<Span>, result: Vec<(Style, &str)>) {
    for &(ref style, text) in result.iter() {
        highlighted_line.push(Span::raw(text.to_string()).fg(Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b)));
    }
}

#[inline]
fn start_slice_spans(highlighted_line: &mut Vec<Span>, result: Vec<(Style, &str)>, start: usize) {
    let mut counter = 0;

    for &(ref style, text) in result.iter() {
        let mut chars = String::new();

        for char in text.chars() {
            if counter >= start {
                chars.push(char);
            }

            counter += 1;
        }

        highlighted_line.push(Span::raw(chars).fg(Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b)));
    }
}

#[inline]
fn start_end_slice_spans(highlighted_line: &mut Vec<Span>, result: Vec<(Style, &str)>, start: usize, end: usize) {
    let mut counter = 0;

    for &(ref style, text) in result.iter() {
        let mut chars = String::new();

        for char in text.chars() {
            if counter >= start {
                if counter < end {
                    chars.push(char);
                }
                else {
                    break;
                }
            }

            counter += 1;
        }

        highlighted_line.push(Span::raw(chars).fg(Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b)));
    }
}