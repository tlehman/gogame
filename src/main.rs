use std::fmt;
use std::cmp::Eq;
use std::cmp::PartialEq;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Color { Empty, Black, White }

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Node {
    x: i16,
    y: i16,
    c: Color
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Edge(Node, Node);

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str = match self { Empty => "", White => "white", Black => "black", };
        write!(f, "{}", s)
    }
}

impl Node {
    fn new(x: i16, y: i16, c: Color) -> Node {
        Node { x, y, c }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"({},{})\" [pos=\"{},{}!\"", self.x, self.y, self.x, self.y);
        match self.c {
            Color::Black => write!(f, ",style=filled,fillcolor=\"#666666\"];"),
            Color::White => write!(f, ",style=filled,fillcolor=\"white\"];"),
            Color::Empty => write!(f, "];")
        };
        write!(f, "\n")
    }
}


impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"({},{})\" -- \"({},{})\";", self.0.x, self.0.y, self.1.x, self.1.y)
    }
}

impl Graph {
    fn new() -> Graph {
        Graph { edges: vec![], nodes: vec![] }
    }
    
    fn add_node(&mut self, node: (i16, i16), color: Color) {
        let mut to_add_node = true;
        let mut to_add_edge = false;
        let mut neighbors = vec![];
        for i in 0..self.nodes.len() {
            let n = &mut self.nodes[i];
            let is_dupe = n.x == node.0 && n.y == node.1;
            if is_dupe {
                to_add_node = false;
                return 
            }

            if n.c == color {
                let vertical_neighbor = (n.x == node.0) && (n.y - node.1).abs() == 1;
                let horizontal_neighbor = (n.y == node.1) && (n.x - node.0).abs() == 1;
                if vertical_neighbor || horizontal_neighbor {
                    to_add_edge = true;
                    neighbors.push(n.clone());
                }
            }
        }
        if to_add_node {
            self.nodes.push(Node::new(node.0, node.1, color.clone()));
        }
        if to_add_edge {
            for i in 0..neighbors.len() {
                let a = Node::new(node.0, node.1, color.clone());
                self.edges.push(Edge(a, neighbors[i].clone()));
            }
        }

    }
    fn add_node_black(&mut self, node: (i16, i16)) {
        self.add_node(node, Color::Black);
    }
    fn add_node_white(&mut self, node: (i16, i16)) {
        self.add_node(node, Color::White);
    }

    // TODO: test that only adjacent and same color nodes are added
    fn add_edge_black(&mut self, src: (i16, i16), dst: (i16, i16)) {
        let a = Node::new(src.0, src.1, Color::Black);
        let b = Node::new(dst.0, dst.1, Color::Black);
        self.nodes.push(a);
        self.nodes.push(b);
        let e = Edge(
            Node::new(src.0, src.1, Color::Black),
            Node::new(dst.0, dst.1, Color::Black)
        );

        self.edges.push(e);
    }
    fn add_edge_white(&mut self, src: (i16, i16), dst: (i16, i16)) {
        let a = Node::new(src.0, src.1, Color::White);
        let b = Node::new(dst.0, dst.1, Color::White);
        self.nodes.push(a);
        self.nodes.push(b);
        let e = Edge(
            Node::new(src.0, src.1, Color::White),
            Node::new(dst.0, dst.1, Color::White)
        );

        self.edges.push(e);
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "graph {{\n");
        for i in 0..self.nodes.len() {
            write!(f, "{}\n", self.nodes[i]);
        }
        for i in 0..self.edges.len() {
            write!(f, "{}\n", self.edges[i]);
        }
        write!(f, "}}")
    }
}

fn main() {
    let mut g = Graph::new();
    g.add_node_black((1,5));
    g.add_node_white((1,6));
    g.add_node_black((2,5));
    g.add_node_white((4,6));
    g.add_node_black((1,4));
    g.add_node_white((3,6));
    g.add_node_black((3,5));
    g.add_node_white((2,4));
    g.add_node_black((2,3));
    g.add_node_white((3,4));

    println!("{}", g);
}

#[test]
fn add_node_increases_node_count() {
    let mut g = Graph::new();
    assert_eq!(0, g.nodes.len());
    g.add_node_black((1,5));
    assert_eq!(1, g.nodes.len());

    // check that duplicates are not added
    g.add_node_black((1,5));
    assert_eq!(1, g.nodes.len());
}

#[test]
fn add_node_increases_edge_count() {
    let mut g = Graph::new();
    assert_eq!(0, g.edges.len());
    g.add_node_black((1,5));
    g.add_node_black((2,5));
    assert_eq!(1, g.edges.len());
    // check that duplicates are not added
    g.add_node_black((2,5));
    assert_eq!(1, g.edges.len());
}
