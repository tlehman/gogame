use std::fmt;
use std::collections::HashSet;
use std::collections::VecDeque;
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

fn contained_in(n: &Node, v: &VecDeque<Node>) -> bool {
    for i in 0..v.len() {
        if n == &v[i] {
            return true;
        }
    }
    false
}

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

    fn neighbors_of(&self, n: Node) -> Vec<Node> {
        let mut neighbors = vec![];
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];

            if node.c == n.c {
                let vertical_neighbor = (n.x == node.x) && (n.y - node.y).abs() == 1;
                let horizontal_neighbor = (n.y == node.y) && (n.x - node.x).abs() == 1;
                if vertical_neighbor || horizontal_neighbor {
                    neighbors.push(node.clone());
                }
            }
        }
        neighbors
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

    fn find_component_containing(&mut self, u: Node) -> HashSet<Node> {
        // compute connected components and capture any with no liberties
        // use a breadth-first search from u
        let mut reached: VecDeque<Node> = VecDeque::new();
        let mut searched: HashSet<Node> = HashSet::new();

        reached.push_back(u.clone());
        searched.insert(u.clone());
        while reached.len() > 0 {
            let option_v = reached.pop_front();
            match option_v {
                Some(v) => {
                    // neighbors of v not in (S U R) add to R
                    if !searched.contains(&v) && !contained_in(&v, &reached) {
                        reached.push_back(v.clone());
                    }
                    searched.insert(v);
                },
                None => { println!("reached set is full"); }
            }
        }
        /* visit node x by adding to queue, search node by following
         * neighbors until component is searched starting from x.
         *
         * compute non-adjacent spots "liberties"
         */
        searched
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
    g.add_node_black((4,4));
    g.add_node_white((1,3));
    g.add_node_black((3,3));
    g.add_node_white((4,5));
    g.add_node_black((4,3));

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

#[test]
fn capture_single_works() {
    // capturing a single piece should work, assuming 22 is white
    // and 12, 22, 32 and 32 are black, then the black pieces
    // should capture the white 22 piece and remove it from the board.
    let mut g = Graph::new();
    /*
     *     21
     *  12 22 32
     *     23
     *
     *
     */
    g.add_node_black((1,2));
    g.add_node_black((2,1));
    g.add_node_black((3,2));
    g.add_node_black((2,3));
    assert_eq!(4, g.nodes.len());
    g.add_node_white((2,2));
    //assert_eq!(4, g.nodes.len());
}

//#[test]
fn find_component_doubleton() {
    let mut g = Graph::new();
    g.add_node_black((1,2));
    g.add_node_white((5,5));
    g.add_node_black((2,2));
    let u = Node::new(1,2, Color::Black);
    let u_component = g.find_component_containing(u);
    assert_eq!(2, u_component.len());
}

#[test]
fn contained_in_works() {
    let mut s: VecDeque<Node> = VecDeque::new();
    let n = Node::new(1,2,Color::Black);
    assert_eq!(false, contained_in(&n, &s));
    s.push_front(n.clone());
    assert_eq!(true, contained_in(&n, &s));
}

#[test]
fn neighbors_of_works() {
    // capturing a single piece should work, assuming 22 is white
    // and 12, 22, 32 and 32 are black, then the black pieces
    // should capture the white 22 piece and remove it from the board.
    let mut g = Graph::new();
    /*
     *   x 21
     *  12 y 32
     *     23
     *
     *
     */
    g.add_node_black((1,1));
    g.add_node_white((2,2));
    g.add_node_black((1,2));
    g.add_node_white((2,3));
    g.add_node_black((2,1));
    g.add_node_white((3,2));

    let x = Node::new(1,1,Color::Black);
    let neighbors_x = g.neighbors_of(x);
    let expected_neighbors_x = vec![
        Node::new(1,2,Color::Black),
        Node::new(2,1,Color::Black),
    ];
    assert_eq!(neighbors_x, expected_neighbors_x);

    let y = Node::new(2,2,Color::White);
    let neighbors_y = g.neighbors_of(y);
    let eypected_neighbors_y = vec![
        Node::new(2,3,Color::White),
        Node::new(3,2,Color::White),
    ];
    assert_eq!(neighbors_y, eypected_neighbors_y);
}
