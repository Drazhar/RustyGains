use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::data::DB;

pub fn create<'a>(total_width: usize, db: &DB) -> Vec<Line<'a>> {
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
            .expect("Never too far back into the past");

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
    iso_week_line.spans[0].style = Style::default().add_modifier(Modifier::DIM);

    let mut weekday = create_empty_weeks(["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"], width);

    // Parse activity log into weekdays

    let activity_log = db.get_activity_log(None);
    let now_naive = Local::now().naive_local();
    let now_normalized = normalize_date(now_naive);
    for a in activity_log {
        let date = NaiveDateTime::from_timestamp_millis(a.date as i64 * 1000).unwrap();
        let date_normalized = normalize_date(date);
        let d_weeks = (now_normalized - date_normalized).num_weeks();
        if d_weeks <= width as i64 {
            let row = date.weekday().number_from_monday() - 1;
            let col: i64 = width as i64 - d_weeks;
            weekday[row as usize][col as usize] = Span::styled(
                match a.intensity {
                    0 => "░",
                    1 => "▒",
                    2 => "▓",
                    _ => "█",
                },
                Style::default().fg(a.activity.color.into()),
            );
        }
    }

    // OMG ist das viel Arbeit

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

/// Returns the date of the beginning (Monday) of the week
fn normalize_date(date: NaiveDateTime) -> NaiveDate {
    let weekday = date.weekday().number_from_monday();
    let date = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap();
    date.checked_sub_signed(Duration::days((weekday - 1) as i64))
        .unwrap()
}
