#![allow(non_snake_case)]

use macroquad::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

const NODE_NUM: u32 = 100;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    x: f64,
    y: f64,
}
impl From<[f64;2]> for Node {
    fn from(value: [f64;2]) -> Self {
        Node { 
            x: value[0], 
            y: value[1],
        }
    }
}
impl Node {
    const MAX_DIST: f64 = std::f64::consts::SQRT_2;
    const MAX_VIXIBLE_DIST: Option<f64> = None;
    const SIZE: Option<f32> = None;

    fn get_size() -> f32 {
        if let Some(value) = Node::SIZE  {
            value
        } else {
            (1. / NODE_NUM as f32) * (100. * (NODE_NUM as f32).sqrt())
        }
    }

    fn get_max_visible_dist() -> f64 {
        if let Some(value) = Node::MAX_VIXIBLE_DIST  {
            value
        } else {
            1. / 2_f64.powf((NODE_NUM as f64).log10() + 1.)
        }
    }

    fn distance(n1: &Node, n2: &Node) -> f64 {
        ((n1.x - n2.x).powi(2) + (n1.y - n2.y).powi(2)).sqrt()
    }

    fn draw_node(&self) {
        draw_circle(self.x as f32 * screen_width(), self.y as f32 * screen_height(), Node::get_size()/2., PURPLE)
    }
}

#[derive(Clone)]
struct Graph {
    nodes: HashMap<usize, Node>,
    node_count: usize,
    relationships: Vec<(usize, usize, f64)>,
    picked_up: Option<usize>
}
impl<'a> Graph {
    fn new() -> Self {
        Graph{ nodes: HashMap::new(), node_count: 0, relationships: Vec::new(), picked_up: None }
    }

    fn push<T: Into<Node>>(&mut self, node_like: T) {
        let node = node_like.into();

        let node_count = self.node_count;
        self.nodes.insert(node_count, node); 
        self.calc_rels(&node_count);
        self.node_count += 1;
        
    }


    fn calc_rels(&mut self, node_id: &usize) {
        for (other_id, other) in self.nodes.iter() {
            if other_id != node_id {
                let node = self.nodes.get(&node_id).unwrap();
                let max_visible_dist = Node::get_max_visible_dist();
                let dist = Node::distance(&node, other,);
                let relative_dist = (Node::MAX_DIST - dist) / Node::MAX_DIST;
                let rel_intensity = ((relative_dist - (1. - max_visible_dist)).clamp(0., 1.)) / max_visible_dist;
                if dist < max_visible_dist {
                    self.relationships.push((*node_id, *other_id, rel_intensity))
                }
            }
        }
    }

    fn clear_rels(&mut self, node_id: &usize) {
        self.relationships.retain(|(self_id, other_id, _)| (self_id != node_id && other_id != node_id) )
    }

    fn draw_relationships(&self) {
        for rel in &self.relationships {
            let node = self.nodes.get(&rel.0).unwrap();
            let other = self.nodes.get(&rel.1).unwrap();
            draw_line(
                node.x as f32 * screen_width(), 
                node.y as f32 * screen_height(), 
                other.x as f32 * screen_width(), 
                other.y as f32 * screen_height(),
                Node::get_size() / 3., 
                [1., 0., 0., rel.2 as f32].into()
            );
        }
    }

    fn draw_nodes(&self) {
        for (_, node) in self.nodes.iter() {
            node.draw_node()
        }
    }

    fn populate_random(count: u32) -> Self {
        let mut graph = Graph::new();

        for _ in 0..count {
            graph.push([ 
                macroquad::rand::rand() as f64 / std::u32::MAX as f64, 
                macroquad::rand::rand() as f64 / std::u32::MAX as f64,
            ])
        }

        return graph;
    }

    fn handle_dragging(&mut self) {
        let mouse_pos = (mouse_position().0 / screen_width(), mouse_position().1 / screen_height());
        if is_mouse_button_pressed(MouseButton::Left) {
            for (id, node) in &self.nodes {
                if (mouse_pos.0 - node.x as f32).powi(2) + (mouse_pos.1 - node.y as f32).powi(2) < (Node::get_size() / screen_width() / 2.).powi(2) {
                    self.picked_up = Some(*id);
                }
            }
            #[cfg(feature = "cheap_dragging")]{
                if let Some(id) = self.picked_up {
                    self.clear_rels(&id);
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            #[cfg(feature = "cheap_dragging")]{
                if let Some(id) = self.picked_up {
                    self.calc_rels(&id);
                }
            }
            if let Some(_) = self.picked_up {
                self.picked_up = None;
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(id) = self.picked_up {

                #[cfg(not(feature = "cheap_dragging"))]
                {
                    self.clear_rels(&id);
                    self.calc_rels(&id);
                }
                
                let node = self.nodes.get_mut(&id).unwrap();
                node.x = mouse_pos.0 as f64;
                node.y = mouse_pos.1 as f64;
            };
        }
    }
}

use std::time::Instant;
#[macroquad::main("eNeRGy")]
async fn main() {
    //let mut graph = Graph{ nodes: vec![ [0.4, 0.5].into(), [0.5, 0.4].into(), [0.6, 0.5].into(), [0.5, 0.6].into() ] };
    let mut graph = Graph::populate_random(NODE_NUM);
    loop {
        graph.handle_dragging();

        //let now = Instant::now();
        graph.draw_relationships();
        //println!("{:?}", now.elapsed());
        graph.draw_nodes();

        if is_mouse_button_pressed(MouseButton::Right) {
            graph.push([
                (mouse_position().0 / screen_width()) as f64,
                (mouse_position().1 / screen_height()) as f64,
            ])
        }

        next_frame().await
    }
}