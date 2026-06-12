use petgraph::graph::{Graph};
use petgraph::Directed;

fn main() {
    // Создаем пример направленного графа
    let mut graph = Graph::<(), (), Directed>::new();

    let v1 = graph.add_node(());
    let v2 = graph.add_node(());
    let v3 = graph.add_node(());
    let v4 = graph.add_node(());
    let v5 = graph.add_node(());
    let _v6 = graph.add_node(());
    let v7 = graph.add_node(());
    let v8 = graph.add_node(());

    // Добавляем ребра
    graph.add_edge(v1, v5, ());
    graph.add_edge(v1, v7, ());
    graph.add_edge(v1, v8, ());

    graph.add_edge(v2, v3, ());
    graph.add_edge(v2, v8, ());
    graph.add_edge(v2, v8, ());
    graph.add_edge(v2, v7, ());

    graph.add_edge(v3, v2, ());
    graph.add_edge(v3, v8, ());
    graph.add_edge(v3, v4, ());

    graph.add_edge(v4, v3, ());
    graph.add_edge(v4, v8, ());

    graph.add_edge(v5, v1, ());

    // As 'start' & 'end' parsing as 1 node,
    // then need give lib 'start' & 'end' ___ 'end' & 'start' 2 times
    graph.add_edge(v7, v7, ());
    graph.add_edge(v7, v7, ());

    graph.add_edge(v7, v7, ());
    graph.add_edge(v7, v7, ());

    //

    graph.add_edge(v7, v8, ());
    graph.add_edge(v7, v1, ());
    graph.add_edge(v7, v2, ());

    graph.add_edge(v8, v1, ());
    graph.add_edge(v8, v2, ());
    graph.add_edge(v8, v3, ());
    graph.add_edge(v8, v4, ());
    graph.add_edge(v8, v7, ());


    // Getting adj-matrix
    let node_count = graph.node_count();
    let mut adjacency_matrix = vec![vec![0; node_count]; node_count];

    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        adjacency_matrix[source.index()][target.index()] += 1;
    }


    println!("Adjacency Matrix:");

    for row in &adjacency_matrix { // Output matrix
        for value in row { print!("{} ", value); }

        println!();
    }
}
