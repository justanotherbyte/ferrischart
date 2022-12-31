mod charts;

#[cfg(test)]
mod tests {
    use crate::charts::scatter::ScatterGraph;

    #[test]
    fn scatter_builder() {
        let graph = ScatterGraph::build()
            .set_axis_text("hello", "how u")
            .draw("test.png");
    }
}
