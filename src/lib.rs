pub mod charts;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::{charts::scatter::ScatterGraph, error::ChartResult};

    #[test]
    fn scatter_builder_numbers() -> ChartResult<()> {
        ScatterGraph::build()
            .set_title("Number Labels")
            .set_axis_text("GCSE Grades", "IB Grades")
            .set_labels(
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]
            )
            .load_data(vec![(1.0, 2.0), (5.0, 4.0), (1.0, 2.0)])
            .draw("tests/numbers.png")?;

        Ok(())
    }
    // #[test]
    // fn scatter_builder_text() -> ChartResult<()> {
    //     ScatterGraph::build()
    //         .set_title("Text Labels")
    //         .set_axis_text("GCSE Grades", "IB Grades")
    //         .set_labels(
    //             vec!["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"],
    //             vec!["One", "Two", "Three", "Four", "Five", "Six", "Seven"]
    //         )
    //         .draw("tests/text.png")?;

    //     Ok(())
    // }
}
