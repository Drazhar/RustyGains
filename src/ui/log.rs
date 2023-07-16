use chrono::{DateTime, Duration, Local, Months, NaiveDateTime};
use ratatui::widgets::TableState;

use crate::data::{exercise::Exercise, DB};

pub struct LogState {
    pub active_area: LogArea,
    pub selected_activity: usize,
    pub selected_activity_row: usize,
    pub selected_time: TimeSelection,
    pub date: DateTime<Local>,
    pub intensity: u8,
    pub comment: String,
    pub exercises: Vec<ExerciseLog>,
    pub exercise_selection: ExerciseSelection,
    pub timer: Timer,
    pub table: TableState,
    pub table_size: usize,
}

#[derive(Default)]
pub struct Timer {
    started: bool,
    start_time: NaiveDateTime,
    break_time: NaiveDateTime,
}

impl Timer {
    pub fn is_started(&self) -> bool {
        self.started
    }

    pub fn toggle(&mut self) {
        if !self.started {
            self.start_time = chrono::Local::now().naive_local();
            self.break_time = self.start_time;
        }
        self.started = !self.started;
    }

    pub fn round(&mut self) {
        if self.started {
            self.break_time = chrono::Local::now().naive_local();
        }
    }

    pub fn get_elapsed(&self) -> Duration {
        if !self.started {
            return Duration::hours(0);
        }
        let now = chrono::Local::now().naive_local();
        now - self.start_time
    }

    pub fn get_round(&self) -> Duration {
        if !self.started {
            return Duration::hours(0);
        }
        let now = chrono::Local::now().naive_local();
        now - self.break_time
    }
}

pub struct ExerciseSelection {
    pub exercise_number: usize,
    pub element: ExerciseElement,
}

pub enum ExerciseElement {
    Name,
    Weight,
    Break,
    Set(usize),
    Intensity,
}

impl Default for LogState {
    fn default() -> Self {
        Self {
            active_area: LogArea::Activity,
            selected_activity: 0,
            selected_activity_row: 0,
            selected_time: TimeSelection::Day,
            date: Local::now(),
            intensity: 0,
            comment: String::from(""),
            exercises: Vec::default(),
            exercise_selection: ExerciseSelection {
                exercise_number: 0,
                element: ExerciseElement::Name,
            },
            timer: Timer::default(),
            table: TableState::default().with_selected(Some(0)),
            table_size: 0,
        }
    }
}

impl LogState {
    pub fn offset_selected_activity_row(&mut self, n: isize) {
        let row = &mut self.selected_activity_row;
        let row_count = 4;
        if n > 0 {
            if row < &mut (row_count - 1) {
                *row += 1;
            } else {
                *row = 0;
            }
        } else if *row == 0 {
            *row = row_count - 1;
        } else {
            *row -= 1;
        }
    }

    pub fn increase_time(&mut self) {
        self.date = match self.selected_time {
            TimeSelection::Day => self.date.checked_add_signed(Duration::days(1)).unwrap(),
            TimeSelection::Month => self.date.checked_add_months(Months::new(1)).unwrap(),
            TimeSelection::Year => self.date.checked_add_months(Months::new(12)).unwrap(),
            TimeSelection::Hour => self.date.checked_add_signed(Duration::hours(1)).unwrap(),
        }
    }

    pub fn decrease_time(&mut self) {
        self.date = match self.selected_time {
            TimeSelection::Day => self.date.checked_sub_signed(Duration::days(1)).unwrap(),
            TimeSelection::Month => self.date.checked_sub_months(Months::new(1)).unwrap(),
            TimeSelection::Year => self.date.checked_sub_months(Months::new(12)).unwrap(),
            TimeSelection::Hour => self.date.checked_sub_signed(Duration::hours(1)).unwrap(),
        }
    }

    pub fn increase_intensity(&mut self) {
        if self.intensity < 3 {
            self.intensity += 1;
        }
    }

    pub fn decrease_intensity(&mut self) {
        if self.intensity > 0 {
            self.intensity -= 1;
        }
    }

    pub fn next_exercise_selection(&mut self) {
        let element = &mut self.exercise_selection.element;

        *element = match element {
            ExerciseElement::Name => ExerciseElement::Weight,
            ExerciseElement::Weight => ExerciseElement::Break,
            ExerciseElement::Break => ExerciseElement::Set(0),
            ExerciseElement::Set(i) => {
                let exercise_number = &self.exercise_selection.exercise_number;
                let set_count = self.exercises[*exercise_number].reps.len();

                if *i == set_count {
                    ExerciseElement::Intensity
                } else {
                    ExerciseElement::Set(*i + 1)
                }
            }
            ExerciseElement::Intensity => ExerciseElement::Name,
        };
    }

    pub fn prev_exercise_selection(&mut self) {
        let element = &mut self.exercise_selection.element;

        *element = match element {
            ExerciseElement::Name => ExerciseElement::Intensity,
            ExerciseElement::Weight => ExerciseElement::Name,
            ExerciseElement::Break => ExerciseElement::Weight,
            ExerciseElement::Set(i) => {
                if *i == 0 {
                    ExerciseElement::Break
                } else {
                    ExerciseElement::Set(*i - 1)
                }
            }
            ExerciseElement::Intensity => ExerciseElement::Set(
                self.exercises[self.exercise_selection.exercise_number]
                    .reps
                    .len(),
            ),
        };
    }

    pub fn horizontal_move_exercise_area(&mut self, dir: i8) {
        if self.exercise_selection.exercise_number < self.exercises.len() {
            if dir > 0 {
                self.next_exercise_selection();
            } else {
                self.prev_exercise_selection();
            }
        }
    }

    pub fn vertical_move_exercise_area(&mut self, dir: i8) {
        let ex_number = &mut self.exercise_selection.exercise_number;
        if dir < 0 {
            if *ex_number < self.exercises.len() {
                *ex_number += 1;
            }
        } else if *ex_number > 0 {
            *ex_number -= 1;
        }
    }

    pub fn increase_selection(&mut self, ex_list: Vec<Exercise>, db: &DB) {
        if self.exercise_selection.exercise_number == self.exercises.len() {
            self.exercises.push(ExerciseLog::new(db));
        } else {
            let ex_number = &self.exercise_selection.exercise_number;
            let exercise = &mut self.exercises[*ex_number];
            match self.exercise_selection.element {
                ExerciseElement::Name => {
                    let ex_index = ex_list
                        .iter()
                        .position(|e| e.id == exercise.exercise.id)
                        .unwrap();
                    exercise.exercise = ex_list[(ex_index + 1) % ex_list.len()].clone();
                }
                ExerciseElement::Weight => exercise.weight += 0.25,
                ExerciseElement::Break => exercise.breaks += 0.5,
                ExerciseElement::Set(s) => {
                    if s >= exercise.reps.len() {
                        let default_reps = if exercise.reps.is_empty() {
                            8
                        } else {
                            exercise.reps[0]
                        };
                        exercise.reps.push(default_reps);
                        self.exercise_selection.element = ExerciseElement::Set(s + 1);
                    } else {
                        exercise.reps[s] += 1;
                    }
                }
                ExerciseElement::Intensity => {
                    if exercise.effort < 9 {
                        exercise.effort += 1;
                    }
                }
            }
        }
    }

    pub fn decrease_selection(&mut self, ex_list: Vec<Exercise>) {
        if self.exercise_selection.exercise_number == self.exercises.len() {
            if !self.exercises.is_empty() {
                self.exercises.pop();
                self.exercise_selection.exercise_number -= 1;
            }
        } else {
            let ex_number = &self.exercise_selection.exercise_number;
            let exercise = &mut self.exercises[*ex_number];
            match self.exercise_selection.element {
                ExerciseElement::Name => {
                    let ex_index = ex_list
                        .iter()
                        .position(|e| e.id == exercise.exercise.id)
                        .unwrap();
                    exercise.exercise =
                        ex_list[(ex_index + ex_list.len() - 1) % ex_list.len()].clone();
                }
                ExerciseElement::Weight => exercise.weight -= 0.25,
                ExerciseElement::Break => exercise.breaks -= 0.5,
                ExerciseElement::Set(s) => {
                    if s < exercise.reps.len() {
                        if exercise.reps[s] == 0 && exercise.reps.len() > 1 {
                            exercise.reps.remove(s);
                        } else if exercise.reps[s] > 0 {
                            exercise.reps[s] -= 1;
                        }
                    } else if exercise.reps.len() > 1 {
                        exercise.reps.pop();
                        self.exercise_selection.element = ExerciseElement::Set(s - 1);
                    }
                }
                ExerciseElement::Intensity => {
                    if exercise.effort > 0 {
                        exercise.effort -= 1;
                    }
                }
            }
        }
    }
}

#[derive(PartialEq)]
pub enum LogArea {
    Activity,
    Exercise,
}

pub enum TimeSelection {
    Day,
    Month,
    Year,
    Hour,
}

impl TimeSelection {
    pub fn next(&mut self) {
        *self = match self {
            TimeSelection::Day => TimeSelection::Month,
            TimeSelection::Month => TimeSelection::Year,
            TimeSelection::Year => TimeSelection::Hour,
            TimeSelection::Hour => TimeSelection::Day,
        };
    }
    pub fn prev(&mut self) {
        *self = match self {
            TimeSelection::Day => TimeSelection::Hour,
            TimeSelection::Month => TimeSelection::Day,
            TimeSelection::Year => TimeSelection::Month,
            TimeSelection::Hour => TimeSelection::Year,
        };
    }
}

#[derive(Default)]
pub struct ExerciseLog {
    pub exercise: Exercise,
    pub weight: f64,
    pub breaks: f64,
    pub reps: Vec<u32>,
    pub effort: u8,
}

impl ExerciseLog {
    pub fn new(db: &DB) -> Self {
        match db.get_last_exercise_log() {
            Ok(ex) => ex,
            Err(_) => {
                let exercises = db.get_exercises();
                ExerciseLog {
                    exercise: exercises[0].clone(),
                    ..Default::default()
                }
            }
        }
    }
}
