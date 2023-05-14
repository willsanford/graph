use std::{collections::HashMap, mem::uninitialized};
use std::vec::Vec;

use log::{debug, error, info, warn};
use num_traits::Num;

pub struct DefaultEdgeProp {
    weight: i32,
}
impl Default for DefaultEdgeProp {
    fn default() -> Self {
        DefaultEdgeProp { weight: 0 }
    }
}
impl Clone for DefaultEdgeProp {
    fn clone(&self) -> Self {
        DefaultEdgeProp {
            weight: self.weight,
        }
    }
}

pub struct DefaultVertexProp {}
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

pub struct AdjList<VertexProp: Default + Clone, EdgeProp: Default + Clone> {
    vertices: Vec<Vec<usize>>,
    e_map: HashMap<(usize, usize), EdgeProp>,
    v_map: HashMap<usize, VertexProp>,
    capacity: usize,
    max_capacity: usize,
    directed: bool,
}

impl<VertexProp, EdgeProp> AdjList<VertexProp, EdgeProp>
where
    VertexProp: Default + Clone,
    EdgeProp: Default + Clone,
{
    pub fn new() -> AdjList<VertexProp, EdgeProp> {
        AdjList {
            vertices: Vec::new(),
            e_map: HashMap::new(),
            v_map: HashMap::new(),
            capacity: 0,
            max_capacity: usize::MAX,
            directed: false,
        }
    }

    pub fn from_capacity(cap: usize) -> AdjList<VertexProp, EdgeProp> {
        AdjList {
            vertices: vec![vec![]; cap], 
            e_map: HashMap::new(),
            v_map: HashMap::new(),
            capacity: cap,
            max_capacity: usize::MAX,
            directed: false,
        }

    }

    pub fn add_vertex(&mut self) -> usize {
        self.vertices.push(vec![]);
        self.capacity += 1;
        self.capacity        
    }

    pub fn add_edge(&mut self, u: usize, v:usize, prop: EdgeProp) {
        // Check that the added vertices are already in the adjlist
        // TODO :: if they are not, automatically add them
        if u > self.capacity || v > self.capacity {
           log::error!("Adding a edge with vertices outside the adjacency list range");
           log::error!("Attempting to all ({}, {}) to a graph with capacity {}", u, v, self.capacity);  
           return;
        } 

        if !self.vertices[u].contains(&v){
            self.vertices[u].push(v);
        }            
        self.e_map.insert((u, v), prop.clone());
        // Return early if we are dealing with a directed graph
        if self.directed {return;}
        if !self.vertices[v].contains(&u){
            self.vertices[v].push(u);
        }
        self.e_map.insert((v, u), prop.clone());
    }

    pub fn add_default_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v, EdgeProp::default());
    }

    pub fn get_out_egdes(&mut self, v: usize) -> Option<&Vec<usize>> {
        self.vertices.get(v)
    }

    pub fn remove_vertex(&mut self, v: usize) {
        // Note - this invalidates all the other vertex descriptors 
        // 1. Remove the vertex in the list
        // 2. Iterate through the edge list and find all edges over v decrement
        // 3. Reduce capacity
        unimplemented!()
    }

    pub fn find_vertex(&mut self, f: fn(&VertexProp) -> bool) -> Option<usize> {
        (0..self.capacity).into_iter().find(|&v| f(self.get_v_prop(v).unwrap()))
    }

    pub fn add_vertex_prop(&mut self, v: usize, prop: VertexProp) {
        self.v_map.insert(v, prop.clone());
    }

    pub fn add_edge_prop(&mut self, u: usize, v: usize, prop: EdgeProp) {
        self.e_map.insert((u, v), prop);
    }

    pub fn get_v_prop(&mut self, v: usize) -> Option<&VertexProp> {
        self.v_map.get(&v)
    }

    pub fn get_e_prop(&mut self, u: usize, v: usize) -> Option<&EdgeProp> {
        self.e_map.get(&(u, v))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_basic_operations() {
        let mut adjlist: AdjList<DefaultVertexProp, DefaultEdgeProp> = AdjList::new();

        let mut v_descriptors: Vec<usize> = Vec::new();
        for _ in 0..4 {
            let vd = adjlist.add_vertex();
            v_descriptors.push(vd);
        }

        assert_eq!(adjlist.vertices.len(), 4);
        assert_eq!(adjlist.capacity, 4);

    }

    #[test]
    fn vertex_property_tests() {
        #[derive(Default, Clone)]
        struct VProp<'a> {
            id: &'a str,
            color: &'a str,
        }

        let mut adjlist: AdjList<VProp, DefaultEdgeProp> = AdjList::new();
        let id0: &str = "id1";
        let id1: &str = "id2";
        let color0: &str = "color1";
        let color1: &str = "color2";
        let vp0: VProp = VProp {
            id: id0,
            color: color0,
        };
        let vp1: VProp = VProp {
            id: id1,
            color: color1,
        };

        let v_descriptor_0: usize = adjlist.add_vertex();
        let v_descriptor_1: usize = adjlist.add_vertex();

        adjlist.add_vertex_prop(v_descriptor_0, vp0);
        adjlist.add_vertex_prop(v_descriptor_1, vp1);
        assert_eq!(id0, adjlist.get_v_prop(v_descriptor_0).unwrap().id);
        assert_eq!(id1, adjlist.get_v_prop(v_descriptor_1).unwrap().id);
        assert_eq!(color0, adjlist.get_v_prop(v_descriptor_0).unwrap().color);
        assert_eq!(color1, adjlist.get_v_prop(v_descriptor_1).unwrap().color);
    }

    // #[test]
    // fn edge_basic_operations() {
    //     let mut adjlist: AdjList<DefaultVertexProp, DefaultEdgeProp> = AdjList::new();

    //     adjlist.add_vertex(0);
    //     adjlist.add_vertex(1);
    //     adjlist.add_vertex(2);

    //     adjlist.add_default_edge((0, 2));
    //     adjlist.add_default_edge((1, 3));
    //     adjlist.add_default_edge((2, 1));

    //     assert_eq!(adjlist.get_out_egdes(0).unwrap().len(), 2);

    //     let mut adjlist_d: AdjList<DefaultVertexProp, DefaultEdgeProp> = AdjList::new();
    //     adjlist_d.directed = true;
    //     adjlist_d.add_vertex(0);
    //     adjlist_d.add_vertex(1);
    //     adjlist_d.add_vertex(2);

    //     adjlist_d.add_default_edge((0, 2));
    //     adjlist_d.add_default_edge((1, 3));
    //     adjlist_d.add_default_edge((2, 1));

    //     assert_eq!(adjlist_d.get_out_egdes(0).unwrap().len(), 1);
    // }

    // #[test]
    // fn edge_property_tests() {
    //     #[derive(Default, Clone)]
    //     struct EProp<'a> {
    //         id: i32,
    //         color: &'a str,
    //         weight: i32,
    //     }

    //     let mut adjlist: AdjList<DefaultVertexProp, EProp> = AdjList::new();

    //     let color0: &str = "blue";
    //     let color1: &str = "red";
    //     let color2: &str = "green";
    //     let e_prop_0: EProp = EProp {
    //         id: 0,
    //         color: color0,
    //         weight: 14,
    //     };
    //     let e_prop_1: EProp = EProp {
    //         id: 1,
    //         color: color1,
    //         weight: 29,
    //     };
    //     let e_prop_2: EProp = EProp {
    //         id: 2,
    //         color: color2,
    //         weight: 44,
    //     };

    //     adjlist.add_edge((0, 2), e_prop_0);
    //     adjlist.add_edge((1, 3), e_prop_1);
    //     adjlist.add_edge((2, 1), e_prop_2);

    //     assert_eq!(adjlist.get_e_prop((0, 2)).unwrap().id, 1);
    //     assert_eq!(adjlist.get_e_prop((1, 3)).unwrap().color, color2);
    //     assert_eq!(adjlist.get_e_prop((2, 1)).unwrap().weight, 45);

    //     assert!(adjlist.get_e_prop((2, 3)).is_none());
    // }

    // #[test]
    // fn test() {
    //     assert!(true);
    // }
}