use crate::errors::Result;
use crate::models::{Score, Student};
use crate::student_service::SqliteStudentService;
use crate::useful::*;
use chrono::{Duration, NaiveDate};
use plotters::prelude::*;

pub struct SafmedPlotter {
    service: Box<SqliteStudentService>,
}

impl SafmedPlotter {
    pub fn new(service: Box<SqliteStudentService>) -> Self {
        SafmedPlotter { service }
    }

    pub fn plot(student: Student) -> Result<()> {
        let path = "./plot.png";
        let root_area = BitmapBackend::new(path, (1200, 900)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();

        let naive_dates = prev_date(11).unwrap();
        let dates_linstep = naive_dates.clone().step(Duration::days(1));
        let correct = vec![30_i64, 45, 73, 99, 30_i64, 45, 73, 99, 30_i64, 45, 73, 99];
        let incorrect = vec![8_i64, 11, 10, 7, 8_i64, 11, 10, 7, 8_i64, 11, 10, 7];

        let data: Vec<(NaiveDate, i64)> = dates_linstep.clone().values().zip(correct).collect();
        let data_i: Vec<(NaiveDate, i64)> = dates_linstep.clone().values().zip(incorrect).collect();

        let mut ctx = ChartBuilder::on(&root_area)
            .margin(50)
            // .set_label_area_size(LabelAreaPosition::Left, 20)
            // .set_label_area_size(LabelAreaPosition::Bottom, 20)
            // .caption("Safmed Scores for Ben Jones", ("sans-serif", 20))
            .build_cartesian_2d(naive_dates.clone(), (1..100).log_scale())
            .unwrap();

        ctx.configure_mesh().draw().unwrap();

        ctx.draw_series(data.into_iter().map(|point| {
            let c = Circle::new(point, 8.0_f64, GREEN.filled());
            c
        }))
        .unwrap();
        ctx.draw_series(data_i.into_iter().map(|point| {
            let c = Circle::new(point, 6.0_f64, RED.filled());
            c
        }))
        .unwrap();
        Ok(())
    }
}
