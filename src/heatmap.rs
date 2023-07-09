use chrono::{Datelike, Duration};
use tui::{
    style::Style,
    text::{Line, Span},
};

pub fn render<'a>(total_width: usize) -> Vec<Line<'a>> {
    let now = chrono::Local::now();
    let weekday_width: usize = 3;
    let width: usize = total_width - weekday_width;

    let months_extra = 2;
    let years_extra = 3;
    let total_width_usize: usize = total_width;
    let mut iso_weeks = String::with_capacity(total_width_usize);
    let mut months = str::repeat(" ", total_width_usize + months_extra);
    let mut years = str::repeat(" ", total_width_usize + years_extra);
    iso_weeks += &str::repeat(" ", weekday_width);

    for (i, offset) in (0..width).rev().enumerate() {
        let date = now
            .checked_sub_signed(Duration::weeks(offset.try_into().unwrap()))
            .expect("Never too far back into the past!");

        iso_weeks += &format!("{}", date.iso_week().week() % 10);

        // Write the month at it's first week
        if date.day() <= 7 {
            let start: usize = i + weekday_width;
            let end: usize = start + 3;
            months.replace_range(
                start..end,
                match date.month() {
                    1 => "Jan",
                    2 => "Feb",
                    3 => "Mar",
                    4 => "Apr",
                    5 => "May",
                    6 => "Jun",
                    7 => "Jul",
                    8 => "Aug",
                    9 => "Sep",
                    10 => "Oct",
                    11 => "Nov",
                    12 => "Dec",
                    _ => "   ",
                },
            );
        }

        if date.iso_week().week() == 1 {
            let start: usize = i + weekday_width;
            let end: usize = start + 3;
            years.replace_range(start..end, &format!("{}", date.year()));
        }
    }

    years.drain(years.len() - years_extra..years.len());
    months.drain(months.len() - months_extra..months.len());

    let mut iso_week_line = Line::from(iso_weeks);
    iso_week_line.spans[0].style = Style::default().fg(tui::style::Color::DarkGray);

    let weekday = create_empty_weeks(["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"], width);

    // TODO: Parse activity log into weekdays

    let mut result = Vec::with_capacity(3 + 7);
    result.push(Line::from(years));
    result.push(Line::from(months));
    result.push(iso_week_line);
    for wd in weekday {
        result.push(Line::from(wd));
    }

    result
}

fn create_empty_weeks(names: [&str; 7], width: usize) -> Vec<Vec<Span>> {
    let mut result = Vec::with_capacity(names.len());

    for name in names.into_iter() {
        let mut weekday = Vec::with_capacity(width + 1);
        weekday.push(Span::from(name.to_owned() + " "));
        for _ in 0..width {
            weekday.push(Span::from(" "));
        }
        result.push(weekday);
    }

    result
}
