use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::graph::{Edge, UndirectedEdge};
use crate::provide;

pub struct Kruskal {
    sets: Vec<Rc<RefCell<HashSet<usize>>>>,
}

impl Kruskal {
    pub fn init<G, W: Ord, E: Edge<W>>(graph: &G) -> Self
    where
        G: provide::Vertices + provide::Edges<W, E> + provide::Graph<W, E, UndirectedEdge>,
    {
        let vertex_count = graph.vertex_count();

        // let sets = vec![; vertex_count];
        let mut sets = vec![];
        sets.resize_with(vertex_count, || Rc::new(RefCell::new(HashSet::new())));

        for virt_id in 0..vertex_count {
            sets[virt_id].borrow_mut().insert(virt_id);
        }

        Kruskal { sets }
    }

    pub fn execute<'a, G, W: Ord + std::fmt::Debug, E: Edge<W>>(
        mut self,
        graph: &'a G,
    ) -> Vec<(usize, usize)>
    where
        G: provide::Edges<W, E>
            + provide::Vertices
            + provide::Direction
            + provide::Graph<W, E, UndirectedEdge>,
    {
        let mut mst = Vec::<(usize, usize)>::new();

        let id_map = graph.continuos_id_map();

        let mut edges = graph
            .edges()
            .into_iter()
            .filter(|(src_id, dst_id, _)| src_id < dst_id)
            .collect::<Vec<(usize, usize, &E)>>();

        edges.sort_by(|(_, _, e1), (_, _, e2)| e1.get_weight().cmp(e2.get_weight()));

        for (v_real_id, u_real_id, _) in edges {
            let v_virt_id = id_map.virt_id_of(v_real_id);
            let u_virt_id = id_map.virt_id_of(u_real_id);

            if !self.sets[v_virt_id]
                .borrow()
                .eq(&*self.sets[u_virt_id].borrow())
            {
                mst.push((v_real_id, u_real_id));

                let union_set = self.sets[v_virt_id]
                    .borrow()
                    .union(&*self.sets[u_virt_id].borrow())
                    .copied()
                    .collect::<HashSet<usize>>();

                let sharable_set = Rc::new(RefCell::new(union_set));

                for member in sharable_set.borrow().iter() {
                    self.sets[*member] = sharable_set.clone();
                }
            }
        }

        mst
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::Mat;

    #[test]
    fn empty_graph() {
        let graph = MatGraph::init(Mat::<usize>::init());

        let pairs = Kruskal::init(&graph).execute(&graph);

        assert_eq!(pairs.len(), 0);
    }

    #[test]
    fn trivial_directed_graph() {
        //  Given: Graph
        //                5
        //      f ----------------.
        //      |                 |
        //    3 |  1     1     4  |
        //      a --- b --- d --- e
        //    3 |   5 |   2 |   1 |
        //      |     |     |     |
        //      c ----'-----'-----'
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, c, 3.into());
        graph.add_edge(a, f, 3.into());

        graph.add_edge(b, c, 5.into());
        graph.add_edge(b, d, 1.into());

        graph.add_edge(d, c, 2.into());
        graph.add_edge(d, e, 4.into());

        graph.add_edge(e, c, 1.into());
        graph.add_edge(e, f, 5.into());

        let mut tags = std::collections::HashMap::<usize, &'static str>::new();
        tags.insert(a, "a");
        tags.insert(b, "b");
        tags.insert(c, "c");
        tags.insert(d, "d");
        tags.insert(e, "e");
        tags.insert(f, "f");

        let pairs = Kruskal::init(&graph).execute(&graph);

        assert_eq!(pairs.len(), 5);
        assert!(vec![(a, b), (b, a)]
            .iter()
            .any(|pair| pairs.contains(&pair)));

        assert!(vec![(b, d), (d, b)]
            .iter()
            .any(|pair| pairs.contains(&pair)));

        assert!(vec![(e, c), (c, e)]
            .iter()
            .any(|pair| pairs.contains(&pair)));

        assert!(vec![(c, d), (d, c)]
            .iter()
            .any(|pair| pairs.contains(&pair)));

        assert!(vec![(a, f), (f, a)]
            .iter()
            .any(|pair| pairs.contains(&pair)));
    }
}
