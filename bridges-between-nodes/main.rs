use petgraph::prelude::*;
use petgraph::dot::Dot;


fn main() {
    // Суміжні (Інцидентні) вершини - вершини, які зв'язані ребром,
    // Інцидентними ребрами - ребра, які зв'язують вершини

    // Створення нового Направленого графа
    let mut graph = Graph::<&str, &str>::new();

    // Додаємо ВЕРШИНИ
    let v1 = graph.add_node("v1");
    let v2 = graph.add_node("v2");
    let v3 = graph.add_node("v3");
    let v4 = graph.add_node("v4");
    let v5 = graph.add_node("v5");

    // Додаємо РЕБРА
    graph.add_edge(v1, v1, "l1"); // Петля на v1
    graph.add_edge(v1, v2, "l2"); // Ребро між v1 та v2
    graph.add_edge(v2, v3, "l3"); // Ребро між v2 та v3
    graph.add_edge(v3, v3, "l4"); // Петля на v3
    graph.add_edge(v3, v4, "l5"); // Ребро між v3 та v4
    graph.add_edge(v4, v5, "l6"); // Ребро між v4 та v5

    // ####################################################################

    // Визначаємо порядок графа (кіл вершин)
    println!("\n\nGraph order: {}", graph.node_count());
    println!("\n");

    // Формальне викладення у вигляді ТОЧОК та виведення отриманих зв'язків
    let dot = Dot::new(&graph);
    println!("{:?}", dot);

    // Степінь (валентність) вершини - кіл ребер зв'язаних з вершиною
    // Підрахунок валентності кожної вершини
    for node in graph.node_indices() {
        let in_degree = graph.edges_directed(node, petgraph::Incoming).count();
        let out_degree = graph.edges_directed(node, petgraph::Outgoing).count();
        let total_degree = in_degree + out_degree;

        println!(
            "Node {:?}: In valency = {}, Out valency = {}, Global valency = {}",
            graph[node], in_degree, out_degree, total_degree
        );
    }

//(Ізольовані, висячі, кратні) ребра, Петлі
//
//Матрицю суміжностей, множини граф


}


