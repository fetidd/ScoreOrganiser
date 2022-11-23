use crate::errors::Result;
use crate::models::{Score, Student};
use crate::student_service::SqliteStudentService;
use crate::useful::*;
use chrono::{Duration, NaiveDate};
use plotters::prelude::*;

pub trait Plotter {
    pub fn plot(&mut self, student: Student, scores: Vec<impl Score>) -> Result<()>;
}

pub struct SafmedPlotter {
    service: Box<SqliteStudentService>,
    buffer: String
}

impl Plotter for SafmedPlotter {
    pub fn new(service: Box<SqliteStudentService>) -> Self {
        SafmedPlotter { service }
    }

    pub fn plot(&mut self, student: Student, scores: Vec<Score>) -> Result<()> {
        let title = format!("Safmed scores for {} {}", student.first_names.clone(), student.last_name.clone());
        let root_area = SvgBackend::new(self.buffer, (900, 600)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let naive_dates = prev_date(11).unwrap();
        let dates_linstep = naive_dates.clone().step(Duration::days(1));
        let correct: Vec<i64> = scores.iter().map(|s| s.correct).collect();
        let incorrect: Vec<i64> = scores.iter().map(|s| s.incorrect).collect();
        let correct_data: Vec<(NaiveDate, i64)> = dates_linstep.clone().values().zip(correct).collect();
        let incorrect_data: Vec<(NaiveDate, i64)> = dates_linstep.clone().values().zip(incorrect).collect();

        let mut ctx = ChartBuilder::on(&root_area)
            .margin(50)
            .set_label_area_size(LabelAreaPosition::Left, 20)
            .set_label_area_size(LabelAreaPosition::Bottom, 20)
            .caption(&title, ("sans-serif", 20))
            .build_cartesian_2d(naive_dates.clone(), (1..100).log_scale())
            .unwrap();

        ctx.configure_mesh().draw().unwrap();

        ctx.draw_series(correct_data.into_iter().map(|point| {Circle::new(point, 8.0_f64, GREEN.filled())}))?;
        ctx.draw_series(incorrect_data.into_iter().map(|point| {Circle::new(point, 6.0_f64, RED.filled())}))?;
        Ok(())
    }
}
