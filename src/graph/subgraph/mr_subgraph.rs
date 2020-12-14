use crate::provide::{Edges, Graph, Neighbors, Vertices};

use super::{AsSubgraph, Subgraph};
use crate::graph::{Edge, EdgeDir};

/// A subgraph with some vertices elected as root.
///
/// ## Note
/// From now on:
/// * |R|: Means number of roots in the subgraph.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
/// * `G`: **G**raph type that subgraph is representing.
pub struct MultiRootSubgraph<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> {
    roots: Vec<usize>,
    subgraph: Subgraph<'a, W, E, Dir, G>,
}

impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> MultiRootSubgraph<'a, W, E, Dir, G> {
    /// # Arguments
    /// * `graph`: Graph that owns the `edges` and `vertices`.
    /// * `edges`: Edges that are in the subgraph in the format of: (src_id, dst_id, edge).
    /// * `vertices`: Vertices that are in the subgraph.
    /// * `roots`: Roots of the subgraph.
    ///
    /// # Returns
    /// Initialized subgraph containing the specified `edges` and `vertices` and `roots` as roots of the subgraph.
    pub fn init(
        graph: &'a G,
        edges: Vec<(usize, usize, &'a E)>,
        vertices: Vec<usize>,
        roots: Vec<usize>,
    ) -> Self {
        MultiRootSubgraph {
            roots,
            subgraph: Subgraph::init(graph, edges, vertices),
        }
    }

    /// # Returns
    /// Roots of the subgraph.
    ///
    /// # Complexity
    /// O(1)
    pub fn roots(&self) -> &Vec<usize> {
        &self.roots
    }

    /// # Arguments
    /// `vertex_id`: Id of the vertex to be checked wether is a root or not.
    ///
    /// # Returns
    /// * `true`: If vertex with id: `vertex_id` is a root.
    /// * `false`: Otherwise.
    ///
    /// # Complexity
    /// O(|R|)
    pub fn is_root(&self, vertex_id: usize) -> bool {
        self.roots
            .iter()
            .find(|root_id| **root_id == vertex_id)
            .is_some()
    }
}

/// For documentation about each function checkout [`Neighbors`](crate::provide::Neighbors) trait.
/// `MultiRootSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Neighbors
    for MultiRootSubgraph<'a, W, E, Dir, G>
{
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.subgraph.neighbors(src_id)
    }
}

/// For documentation about each function checkout [`Vertices`](crate::provide::Vertices) trait.
/// `MultiRootSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Vertices
    for MultiRootSubgraph<'a, W, E, Dir, G>
{
    fn vertices(&self) -> Vec<usize> {
        self.subgraph.vertices()
    }
}

/// For documentation about each function checkout [`Edges`](crate::provide::Edges) trait.
/// `MultiRootSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Edges<W, E>
    for MultiRootSubgraph<'a, W, E, Dir, G>
{
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.subgraph.edges_from(src_id)
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.subgraph.edges_between(src_id, dst_id)
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.subgraph.edge_between(src_id, dst_id, edge_id)
    }

    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.subgraph.edge(edge_id)
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        self.subgraph.has_any_edge(src_id, dst_id)
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        self.subgraph.edges()
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        self.subgraph.as_directed_edges()
    }

    fn edges_count(&self) -> usize {
        self.subgraph.edges_count()
    }
}

impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> AsSubgraph<W, E>
    for MultiRootSubgraph<'a, W, E, Dir, G>
{
}