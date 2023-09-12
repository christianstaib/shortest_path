use crate::contraction::shortcut_generator::ShortcutGenerator;
use crate::graph::bidirectional_graph::BidirectionalGraph;
use std::sync::RwLock;

use std::rc::Rc;

use super::priority_terms::PriorityTerm;

pub struct EdgeDifferencePriority {
    graph: Rc<RwLock<BidirectionalGraph>>,
}

impl PriorityTerm for EdgeDifferencePriority {
    fn priority(&self, v: u32) -> i32 {
        let shortcut_generator = ShortcutGenerator::new(self.graph.clone());
        let shortcuts = shortcut_generator.naive_shortcuts(v);
        //let shortcuts = shortcut_generator.remove_unnecessary_shortcuts(shortcuts, v);
        shortcuts.len() as i32
    }

    #[allow(unused_variables)]
    fn update(&mut self, v: u32) {}
}

impl EdgeDifferencePriority {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        Self { graph }
    }
}
