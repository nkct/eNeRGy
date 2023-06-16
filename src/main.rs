use macroquad::prelude::*;

#[derive(Debug)]
struct Node {
    x: f64,
    y: f64,
    size: f32,
}
impl From<[f64;2]> for Node {
    fn from(value: [f64;2]) -> Self {
        Node { x: value[0], y: value[1], size: 10. }
    }
}
impl Node {
    const MAX_DIST: f64 = std::f64::consts::SQRT_2;

    fn distance(n1: &Node, n2: &Node) -> f64 {
        ((n1.x - n2.x).powi(2) + (n1.y - n2.y).powi(2)).sqrt()
    }

    fn draw_node(&self) {
        draw_circle(self.x as f32 * screen_width(), self.y as f32 * screen_height(), self.size / 2., PURPLE)
    }

    fn draw_relationship(&self, other: &Node) {
        let brightness = (Node::MAX_DIST - Node::distance(self, other,)) / Node::MAX_DIST - 0.66;
        draw_line(
            self.x as f32 * screen_width(), 
            self.y as f32 * screen_height(), 
            other.x as f32 * screen_width(), 
            other.y as f32 * screen_height(),
            3., 
            [1., 0., 0., brightness as f32].into()
        )
    }
}

struct Graph {
    nodes: Vec<Node>
}
impl Graph {
    fn draw_relationships(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            for other in &self.nodes[i..=self.nodes.len() - 1] {
                node.draw_relationship(other)
            }

        }
    }

    fn draw_nodes(&self) {
        for node in self.nodes.iter() {
            node.draw_node()
        }
    }

    fn populate_random(count: u32) -> Graph {
        let mut graph = Graph { nodes: Vec::new() };

        for _ in 0..count {
            graph.nodes.push([ 
                macroquad::rand::rand() as f64 / std::u32::MAX as f64, 
                macroquad::rand::rand() as f64 / std::u32::MAX as f64
            ].into())
        }

        return graph;
    }
}

#[macroquad::main("eNeRGy")]
async fn main() {
    //let graph = Graph{ nodes: vec![ Node{ x: 0.05, y: 0.05 }, Node{ x: 0.95, y: 0.95 } ] };
    let mut graph = Graph::populate_random(50);
    loop {
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = (mouse_position().0 / screen_width(), mouse_position().1 / screen_height());
            for node in &mut graph.nodes {
                if (mouse_pos.0 - node.x as f32).powi(2) + (mouse_pos.1 - node.y as f32).powi(2) < (node.size / screen_width() / 2.).powi(2) + 0.0005 {
                    node.x = mouse_pos.0 as f64;
                    node.y = mouse_pos.1 as f64;
                }
            }
        }

        graph.draw_relationships();
        graph.draw_nodes();

        next_frame().await
    }
}