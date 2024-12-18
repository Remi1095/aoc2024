use petgraph::prelude::*;
use rustc_hash::FxBuildHasher;

pub type FxDiGraphMap<N, E> = GraphMap<N, E, Directed, FxBuildHasher>;
