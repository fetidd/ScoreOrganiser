use std::sync::Arc;
use crate::errors::{Result, Error};
use crate::services::SafmedScoreService;
use chrono::{Duration, NaiveDateTime, NaiveTime, Days};
use plotters::prelude::*;

pub trait Plotter {
    fn plot(&self, id: &str, buffer: &mut String) -> Result<()>;
}

pub struct SafmedPlotter {
    service: Arc<SafmedScoreService>,
}

impl SafmedPlotter {
    pub fn new(service: Arc<SafmedScoreService>) -> Self {
        SafmedPlotter { service }
    }
}

impl Plotter for SafmedPlotter {
    fn plot(&self, id: &str, mut buffer: &mut String) -> Result<()> {
        let scores = self.service.get_safmed_scores(&id)?;
        let root_area = SVGBackend::with_string(&mut buffer, (900, 700)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
        if scores.len() >= 1 {
            let mut naive_datetimes: Vec<NaiveDateTime> = scores.iter().map(|s| s.date.and_hms_opt(0, 0, 0).unwrap()).collect();
            naive_datetimes.push(NaiveDateTime::new(
                scores.last().unwrap().date.checked_add_days(Days::new(1)).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap()
            ));
            let date_range: RangedDateTime<NaiveDateTime> = (naive_datetimes.first().unwrap().clone()..naive_datetimes.last().unwrap().clone()).into();
            let correct: Vec<i64> = scores.iter().map(|s| s.correct as i64).collect();
            let incorrect: Vec<i64> = scores.iter().map(|s| s.incorrect as i64).collect();
            let correct_data: Vec<(NaiveDateTime, i64)> = naive_datetimes.clone().into_iter().zip(correct).collect();
            let incorrect_data: Vec<(NaiveDateTime, i64)> = naive_datetimes.clone().into_iter().zip(incorrect).collect();
            let mut ctx = ChartBuilder::on(&root_area)
            .margin(50)
            .set_label_area_size(LabelAreaPosition::Left, 20)
            // .set_label_area_size(LabelAreaPosition::Bottom, 20)
            .build_cartesian_2d(date_range.step(Duration::days(1)), (1..100).log_scale())
            .unwrap();
            ctx.configure_mesh().draw().unwrap();
            ctx.draw_series(correct_data.into_iter().map(|point| {Circle::new(point, 8.0_f64, GREEN.filled())})).expect("failed drawing correct");
            ctx.draw_series(incorrect_data.into_iter().map(|point| {Circle::new(point, 6.0_f64, RED.filled())})).expect("failed drawing incorrect");
        } else {
            // draw an empty plot
            let mut ctx = ChartBuilder::on(&root_area)
            .margin(50)
            .set_label_area_size(LabelAreaPosition::Left, 20)
            // .set_label_area_size(LabelAreaPosition::Bottom, 20)
            .build_cartesian_2d(0_i32..1_i32, (1..100).log_scale())
            .unwrap();
            ctx.configure_mesh().draw().unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::{database::{dao::MockDao, Record}, useful::date_from_str};
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
        let service = Arc::new(SafmedScoreService::new(Arc::new(dao)));
        let plotter = SafmedPlotter::new(service);
        let mut buffer = String::new();
        plotter.plot("st1", &mut buffer).unwrap();
        assert_eq!(buffer, String::from("<svg width=\"900\" height=\"700\" viewBox=\"0 0 900 700\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect x=\"0\" y=\"0\" width=\"900\" height=\"700\" opacity=\"1\" fill=\"#FFFFFF\" stroke=\"none\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"649\" x2=\"70\" y2=\"50\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"329\" y1=\"649\" x2=\"329\" y2=\"50\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"589\" y1=\"649\" x2=\"589\" y2=\"50\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"649\" x2=\"849\" y2=\"649\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"559\" x2=\"849\" y2=\"559\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"507\" x2=\"849\" y2=\"507\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"469\" x2=\"849\" y2=\"469\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"440\" x2=\"849\" y2=\"440\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"416\" x2=\"849\" y2=\"416\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"396\" x2=\"849\" y2=\"396\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"379\" x2=\"849\" y2=\"379\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"364\" x2=\"849\" y2=\"364\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"350\" x2=\"849\" y2=\"350\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"350\" x2=\"849\" y2=\"350\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"260\" x2=\"849\" y2=\"260\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"207\" x2=\"849\" y2=\"207\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"170\" x2=\"849\" y2=\"170\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"141\" x2=\"849\" y2=\"141\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"117\" x2=\"849\" y2=\"117\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"97\" x2=\"849\" y2=\"97\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"80\" x2=\"849\" y2=\"80\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"64\" x2=\"849\" y2=\"64\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"50\" x2=\"849\" y2=\"50\"/>\n<line opacity=\"0.1\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"50\" x2=\"849\" y2=\"50\"/>\n<line opacity=\"0.2\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"649\" x2=\"70\" y2=\"50\"/>\n<line opacity=\"0.2\" stroke=\"#000000\" stroke-width=\"1\" x1=\"329\" y1=\"649\" x2=\"329\" y2=\"50\"/>\n<line opacity=\"0.2\" stroke=\"#000000\" stroke-width=\"1\" x1=\"589\" y1=\"649\" x2=\"589\" y2=\"50\"/>\n<line opacity=\"0.2\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"649\" x2=\"849\" y2=\"649\"/>\n<line opacity=\"0.2\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"350\" x2=\"849\" y2=\"350\"/>\n<line opacity=\"0.2\" stroke=\"#000000\" stroke-width=\"1\" x1=\"70\" y1=\"50\" x2=\"849\" y2=\"50\"/>\n<polyline fill=\"none\" opacity=\"1\" stroke=\"#000000\" stroke-width=\"1\" points=\"69,50 69,649 \"/>\n<text x=\"60\" y=\"649\" dy=\"0.5ex\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"9.67741935483871\" opacity=\"1\" fill=\"#000000\">\n1\n</text>\n<polyline fill=\"none\" opacity=\"1\" stroke=\"#000000\" stroke-width=\"1\" points=\"64,649 69,649 \"/>\n<text x=\"60\" y=\"350\" dy=\"0.5ex\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"9.67741935483871\" opacity=\"1\" fill=\"#000000\">\n10\n</text>\n<polyline fill=\"none\" opacity=\"1\" stroke=\"#000000\" stroke-width=\"1\" points=\"64,350 69,350 \"/>\n<text x=\"60\" y=\"50\" dy=\"0.5ex\" text-anchor=\"end\" font-family=\"sans-serif\" font-size=\"9.67741935483871\" opacity=\"1\" fill=\"#000000\">\n100\n</text>\n<polyline fill=\"none\" opacity=\"1\" stroke=\"#000000\" stroke-width=\"1\" points=\"64,50 69,50 \"/>\n<circle cx=\"70\" cy=\"66\" r=\"8\" opacity=\"1\" fill=\"#00FF00\" stroke=\"none\" stroke-width=\"1\"/>\n<circle cx=\"329\" cy=\"66\" r=\"8\" opacity=\"1\" fill=\"#00FF00\" stroke=\"none\" stroke-width=\"1\"/>\n<circle cx=\"589\" cy=\"66\" r=\"8\" opacity=\"1\" fill=\"#00FF00\" stroke=\"none\" stroke-width=\"1\"/>\n<circle cx=\"70\" cy=\"267\" r=\"6\" opacity=\"1\" fill=\"#FF0000\" stroke=\"none\" stroke-width=\"1\"/>\n<circle cx=\"329\" cy=\"267\" r=\"6\" opacity=\"1\" fill=\"#FF0000\" stroke=\"none\" stroke-width=\"1\"/>\n<circle cx=\"589\" cy=\"267\" r=\"6\" opacity=\"1\" fill=\"#FF0000\" stroke=\"none\" stroke-width=\"1\"/>\n</svg>\n"));
    }
}
