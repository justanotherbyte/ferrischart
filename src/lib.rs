pub mod charts;
pub mod error;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{charts::scatter::ScatterGraph, error::ChartResult};

    #[test]
    fn scatter_builder_numbers() -> ChartResult<()> {
        let mut data = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x = rng.gen_range(1.0..9.0);
            let y = rng.gen_range(1.0..7.0);
            data.push((x, y));
        }
        ScatterGraph::build()
            .set_title("GCSE vs IB Grades")
            .set_axis_text("GCSE Grades", "IB Grades")
            .load_data(data)
            .draw("tests/scatter.png")?;

        Ok(())
    }
}
