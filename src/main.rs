use petgraph::Graph;
use petgraph::dot::Dot;

fn main() {
    let mut graph = Graph::<&str, &str>::new();
    let node_a = graph.add_node("A");
    let node_b = graph.add_node("B");
    let node_c = graph.add_node("C");
    let node_d = graph.add_node("D");

    graph.add_edge(node_a, node_b, "ab edge");
    graph.add_edge(node_b, node_c, "bc edge");
    graph.add_edge(node_b, node_d, "bd edge");

    let graphviz_dot = Dot::with_config(&graph, &[]);
    println!("{:?}", graphviz_dot)
}
