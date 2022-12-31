pub mod charts;

#[cfg(test)]
mod tests {
    use crate::charts::scatter::ScatterGraph;

    #[test]
    fn scatter_builder() {
        ScatterGraph::build()
            .set_axis_text("GCSE Grades", "IB Grades")
            .set_labels(
                vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"],
                vec!["1", "2", "3", "4", "5", "6", "7"]
            )
            .draw("test.png");
    }
}
