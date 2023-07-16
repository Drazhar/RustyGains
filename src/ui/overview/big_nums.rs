use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

pub fn big_nums() -> [[Vec<Span<'static>>; 5]; 11] {
    let style = Style::default().add_modifier(Modifier::REVERSED);
    [
        [
            vec![
                Span::from(" "),
                Span::styled("0000", style),
                Span::from(" "),
            ],
            vec![
                Span::styled("00", style),
                Span::from("  "),
                Span::styled("00", style),
            ],
            vec![
                Span::styled("00", style),
                Span::from("  "),
                Span::styled("00", style),
            ],
            vec![
                Span::styled("00", style),
                Span::from("  "),
                Span::styled("00", style),
            ],
            vec![
                Span::from(" "),
                Span::styled("0000", style),
                Span::from(" "),
            ],
        ],
        [
            vec![Span::styled("1111", style), Span::from("  ")],
            vec![
                Span::from("  "),
                Span::styled("11", style),
                Span::from("  "),
            ],
            vec![
                Span::from("  "),
                Span::styled("11", style),
                Span::from("  "),
            ],
            vec![
                Span::from("  "),
                Span::styled("11", style),
                Span::from("  "),
            ],
            vec![Span::styled("111111", style)],
        ],
        [
            vec![
                Span::from(" "),
                Span::styled("2222", style),
                Span::from(" "),
            ],
            vec![
                Span::styled("22", style),
                Span::from("  "),
                Span::styled("22", style),
            ],
            vec![
                Span::from("   "),
                Span::styled("22", style),
                Span::from(" "),
            ],
            vec![
                Span::from("  "),
                Span::styled("22", style),
                Span::from("  "),
            ],
            vec![Span::styled("222222", style)],
        ],
        [
            vec![
                Span::from(" "),
                Span::styled("3333", style),
                Span::from(" "),
            ],
            vec![
                Span::styled("33", style),
                Span::from("  "),
                Span::styled("33", style),
            ],
            vec![
                Span::from("   "),
                Span::styled("33", style),
                Span::from(" "),
            ],
            vec![
                Span::styled("33", style),
                Span::from("  "),
                Span::styled("33", style),
            ],
            vec![
                Span::from(" "),
                Span::styled("3333", style),
                Span::from(" "),
            ],
        ],
        [
            vec![
                Span::styled("44", style),
                Span::from("  "),
                Span::styled("44", style),
            ],
            vec![
                Span::styled("44", style),
                Span::from("  "),
                Span::styled("44", style),
            ],
            vec![Span::styled("444444", style)],
            vec![Span::from("    "), Span::styled("44", style)],
            vec![Span::from("    "), Span::styled("44", style)],
        ],
        [
            vec![Span::styled("555555", style)],
            vec![Span::styled("55", style), Span::from("    ")],
            vec![Span::styled("55555", style), Span::from(" ")],
            vec![Span::from("    "), Span::styled("55", style)],
            vec![Span::styled("55555", style), Span::from(" ")],
        ],
        [
            vec![Span::from(" "), Span::styled("66666", style)],
            vec![Span::styled("66", style), Span::from("    ")],
            vec![Span::styled("66666", style), Span::from(" ")],
            vec![
                Span::styled("66", style),
                Span::from("  "),
                Span::styled("66", style),
            ],
            vec![
                Span::from(" "),
                Span::styled("6666", style),
                Span::from(" "),
            ],
        ],
        [
            vec![Span::styled("777777", style)],
            vec![
                Span::from("   "),
                Span::styled("77", style),
                Span::from(" "),
            ],
            vec![
                Span::from("  "),
                Span::styled("77", style),
                Span::from("  "),
            ],
            vec![
                Span::from(" "),
                Span::styled("77", style),
                Span::from("   "),
            ],
            vec![Span::styled("77", style), Span::from("    ")],
        ],
        [
            vec![
                Span::from(" "),
                Span::styled("8888", style),
                Span::from(" "),
            ],
            vec![
                Span::styled("88", style),
                Span::from("  "),
                Span::styled("88", style),
            ],
            vec![
                Span::from(" "),
                Span::styled("8888", style),
                Span::from(" "),
            ],
            vec![
                Span::styled("88", style),
                Span::from("  "),
                Span::styled("88", style),
            ],
            vec![
                Span::from(" "),
                Span::styled("8888", style),
                Span::from(" "),
            ],
        ],
        [
            vec![
                Span::from(" "),
                Span::styled("9999", style),
                Span::from(" "),
            ],
            vec![
                Span::styled("99", style),
                Span::from("  "),
                Span::styled("99", style),
            ],
            vec![Span::from(" "), Span::styled("99999", style)],
            vec![Span::from("    "), Span::styled("99", style)],
            vec![
                Span::from(" "),
                Span::styled("9999", style),
                Span::from(" "),
            ],
        ],
        [
            vec![Span::from(" ")],
            vec![Span::styled(":", style)],
            vec![Span::from(" ")],
            vec![Span::styled(":", style)],
            vec![Span::from(" ")],
        ],
    ]
}
