use std::ops::{Add, Range};
use std::sync::Arc;

use crate::errors::Result;
use crate::models::{SafmedScore, Student};
use crate::services::ScoreService;
use crate::useful::*;
use chrono::{Duration, NaiveDate, Days, Local, DateTime, NaiveDateTime, Date};
use plotters::coord::ranged1d::AsRangedCoord;
use plotters::prelude::*;

pub trait Plotter {
    fn plot(&mut self, id: &str, buffer: &mut String) -> Result<()>;
}

pub struct SafmedPlotter {
    service: Arc<ScoreService>,
}
impl SafmedPlotter {
    pub fn new(service: Arc<ScoreService>) -> Self {
        SafmedPlotter { service }
    }
}

impl Plotter for SafmedPlotter {
    fn plot(&mut self, id: &str, mut buffer: &mut String) -> Result<()> {
        let student = self.service.get(&id)?;
        let scores = self.service.get_safmed_scores(&id)?;
        let title = format!("Safmed scores for {} {}", student.first_names.clone(), student.last_name.clone());
        let root_area = SVGBackend::with_string(&mut buffer, (900, 600)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let naive_datetimes: Vec<NaiveDateTime> = scores.iter().map(|s| s.date.and_hms_opt(0, 0, 0).unwrap()).collect();
        let correct: Vec<i64> = scores.iter().map(|s| s.correct as i64).collect();
        let incorrect: Vec<i64> = scores.iter().map(|s| s.incorrect as i64).collect();
        let correct_data: Vec<(NaiveDateTime, i64)> = naive_datetimes.clone().into_iter().zip(correct).collect();
        let incorrect_data: Vec<(NaiveDateTime, i64)> = naive_datetimes.clone().into_iter().zip(incorrect).collect();

        let date_range: RangedDateTime<NaiveDateTime> = (naive_datetimes.first().unwrap().clone()..naive_datetimes.last().unwrap().clone()).into();

        let mut ctx = ChartBuilder::on(&root_area)
            .margin(50)
            // .set_label_area_size(LabelAreaPosition::Left, 20)
            // .set_label_area_size(LabelAreaPosition::Bottom, 20)
            .build_cartesian_2d(date_range.step(Duration::days(1)), (1..100).log_scale())
            // .build_cartesian_2d(naive_datetimes.clone().step(Duration::days(1)), (1..100).log_scale())
            .unwrap();

        // ctx.configure_mesh().draw().unwrap();

        ctx.draw_series(correct_data.into_iter().map(|point| {Circle::new(point, 8.0_f64, GREEN.filled())})).expect("failed drawing correct");
        ctx.draw_series(incorrect_data.into_iter().map(|point| {Circle::new(point, 6.0_f64, RED.filled())})).expect("failed drawing incorrect");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::database::{dao::MockDao, Record};
    use super::*;

    #[test]
    fn test_plot() {
        let mut dao = MockDao::new();
        dao.expect_select().withf(move |_,table,_| table == "safmed").times(1).returning(move |_,_,_| {
            Ok(vec![
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-01").unwrap().into())]),
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-02").unwrap().into())]),
                Record::from([("id".into(), "st1".into()), ("correct".into(), 89.into()), ("incorrect".into(), 19.into()), ("date".into(), date_from_str("2021-01-03").unwrap().into())]),
            ])
        });
        let service = Arc::new(ScoreService::new(Arc::new(dao)).unwrap());
        let mut plotter = SafmedPlotter::new(service);
        let mut buffer = String::new();
        plotter.plot("st1", &mut buffer).unwrap();
        assert_eq!(buffer, String::new());
    }
}
