mod algorithm;
// mod structures;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

use num_traits::Num;

struct DefaultEdgeProp {
    weight: i32,
}
impl Default for DefaultEdgeProp {
    fn default() -> Self {
        DefaultEdgeProp { weight: 1 }
    }
}
impl Clone for DefaultEdgeProp {
    fn clone(&self) -> Self {
        DefaultEdgeProp {
            weight: self.weight,
        }
    }
}

struct DefaultVertexProp {}
impl Default for DefaultVertexProp {
    fn default() -> DefaultVertexProp {
        DefaultVertexProp {}
    }
}

impl Clone for DefaultVertexProp {
    fn clone(&self) -> Self {
        DefaultVertexProp {}
    }
}

struct AdjList<VertexProp: Default + Clone, EdgeProp: Default + Clone> {
    v: HashSet<i32>,
    e: HashMap<i32, Vec<i32>>,
    e_map: HashMap<(i32, i32), EdgeProp>,
    v_map: HashMap<i32, VertexProp>,
    directed: bool,
}

impl<VertexProp, EdgeProp> AdjList<VertexProp, EdgeProp>
where
    VertexProp: Default + Clone,
    EdgeProp: Default + Clone,
{
    fn new() -> AdjList<VertexProp, EdgeProp> {
        AdjList {
            v: HashSet::new(),
            e: HashMap::new(),
            e_map: HashMap::new(),
            v_map: HashMap::new(),
            directed: false,
        }
    }
    fn add_vertex(&mut self, v: i32) {
        if self.v.insert(v) {
            // If this is the first time we have seen the vertex, then add the out edge list and a
            // default vertex prop
            self.e.insert(v, Vec::new());
            self.v_map.insert(v, VertexProp::default());
        }
    }

    fn _add_edge(&mut self, e: (i32, i32), prop: EdgeProp) {
        self.add_vertex(e.0);
        self.add_vertex(e.1);
        self.e.get_mut(&e.0).unwrap().push(e.1);
        self.e_map.insert(e, prop.clone());
        if !self.directed {
            self.e.get_mut(&e.1).unwrap().push(e.0);
            self.e_map.insert((e.0, e.1), prop.clone());
        }
    }

    fn add_edge(&mut self, e: (i32, i32), e_prop: EdgeProp) {
        self._add_edge(e, e_prop);
    }

    fn add_default_edge(&mut self, e: (i32, i32)) {
        self._add_edge(e, EdgeProp::default());
    }

    fn get_out_egdes(&mut self, v: i32) -> Option<&Vec<i32>> {
        self.e.get(&v)
    }

    fn remove_vertex(&mut self, v: i32) {
        self.v.remove(&v);
    }

    fn add_vertex_prop(&mut self, v: i32, prop: VertexProp) {
        self.v_map.insert(v, prop);
    }

    fn add_edge_prop(&mut self, e: (i32, i32), prop: EdgeProp) {
        self.e_map.insert(e, prop);
    }

    fn get_v_prop(&mut self, v: i32) -> Option<&VertexProp> {
        self.v_map.get(&v)
    }
    fn get_e_prop(&mut self, e: (i32, i32)) -> Option<&EdgeProp> {
        self.e_map.get(&e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_basic_operations() {
        let mut adjlist: AdjList<DefaultVertexProp, DefaultEdgeProp> = AdjList::new();
        adjlist.add_vertex(1);
        adjlist.add_vertex(2);
        adjlist.add_vertex(3);
        assert_eq!(adjlist.v.len(), 3);

        adjlist.add_vertex(2);
        assert_eq!(adjlist.v.len(), 3);

        adjlist.remove_vertex(2);
        assert_eq!(adjlist.v.len(), 2);
    }

    #[test]
    fn vertex_property_tests() {
        #[derive(Default, Clone)]
        struct VProp<'a> {
            id: &'a str,
            color: &'a str,
        }

        let mut adjlist: AdjList<VProp, DefaultEdgeProp> = AdjList::new();
        let id1: &str = "id1";
        let id2: &str = "id2";
        let color1: &str = "color1";
        let color2: &str = "color2";
        let vp1: VProp = VProp {
            id: id1,
            color: color1,
        };
        let vp2: VProp = VProp {
            id: id2,
            color: color2,
        };
        adjlist.add_vertex(1);

        adjlist.add_vertex_prop(1, vp1);
        assert_eq!(id1, adjlist.get_v_prop(1).unwrap().id);
        assert_ne!(id2, adjlist.get_v_prop(1).unwrap().id);
        assert_eq!(color1, adjlist.get_v_prop(1).unwrap().color);
        assert_ne!(color2, adjlist.get_v_prop(1).unwrap().color);
    }

    #[test]
    fn edge_basic_operations() {
        let mut adjlist: AdjList<DefaultVertexProp, DefaultEdgeProp> = AdjList::new();

        adjlist.add_vertex(1);
        adjlist.add_vertex(2);
        adjlist.add_vertex(3);

        adjlist.add_default_edge((1, 2));
        adjlist.add_default_edge((2, 3));
        adjlist.add_default_edge((3, 1));

        assert_eq!(adjlist.get_out_egdes(1).unwrap().len(), 2);

        let mut adjlist_d: AdjList<DefaultVertexProp, DefaultEdgeProp> = AdjList::new();
        adjlist_d.directed = true;
        adjlist_d.add_vertex(1);
        adjlist_d.add_vertex(2);
        adjlist_d.add_vertex(3);

        adjlist_d.add_default_edge((1, 2));
        adjlist_d.add_default_edge((2, 3));
        adjlist_d.add_default_edge((3, 1));

        assert_eq!(adjlist_d.get_out_egdes(1).unwrap().len(), 1);
    }

    #[test]
    fn edge_property_tests() {
        #[derive(Default, Clone)]
        struct EProp<'a> {
            id: i32,
            color: &'a str,
            weight: i32,
        }

        let mut adjlist: AdjList<DefaultVertexProp, EProp> = AdjList::new();

        let color1: &str = "blue";
        let color2: &str = "red";
        let color3: &str = "green";
        let e_prop_1: EProp = EProp {
            id: 1,
            color: color1,
            weight: 15,
        };
        let e_prop_2: EProp = EProp {
            id: 2,
            color: color2,
            weight: 30,
        };
        let e_prop_3: EProp = EProp {
            id: 3,
            color: color3,
            weight: 45,
        };

        adjlist.add_edge((1, 2), e_prop_1);
        adjlist.add_edge((2, 3), e_prop_2);
        adjlist.add_edge((3, 1), e_prop_3);

        assert_eq!(adjlist.get_e_prop((1, 2)).unwrap().id, 1);
        assert_eq!(adjlist.get_e_prop((2, 3)).unwrap().color, color2);
        assert_eq!(adjlist.get_e_prop((3, 1)).unwrap().weight, 45);

        assert!(adjlist.get_e_prop((3, 3)).is_none());
    }

    #[test]
    fn test() {
        assert!(true);
    }
}
