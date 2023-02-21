use byteorder::{ByteOrder, LittleEndian};

use array2d::Array2D;
use js_sys::Uint8Array;
use log::{debug, Level};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::window;
extern crate console_error_panic_hook;
use std::panic;

mod geometry;

const NODE_DISPLAY_SQUARE_WIDTH: f32 = 8.0;
const DISPLAY_PAN_RATE: f32 = 1.0;

#[wasm_bindgen]
pub fn init_logging() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("Console logging failed to initialise");
}

#[wasm_bindgen]
pub fn get_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub struct GraphFacade {
    graph: GraphDisplay,
}

#[wasm_bindgen]
impl GraphFacade {
    pub fn new(
        node_count: usize,
        spawn_scale: f32,
        display_width: f32,
        display_height: f32,
        display_scale: f32,
    ) -> GraphFacade {
        let layout = GraphLayout::new(node_count, spawn_scale);
        let display = GraphDisplay::new(layout, display_width, display_height, display_scale);
        GraphFacade { graph: display }
    }

    pub fn load_edges(&mut self, chunk_array: Uint8Array) {
        self.graph.layout.load_edges(chunk_array);
    }

    pub fn update_display_size(&mut self, display_width: f32, display_height: f32) {
        self.graph
            .update_display_size(display_width, display_height);
    }

    pub fn update_clipspace_vertices(&mut self) {
        self.graph.update_clipspace_vertices();
    }

    pub fn get_vertices_ptr(&self) -> *const f32 {
        self.graph.get_vertices_ptr()
    }

    pub fn get_vertex_indices_ptr(&self) -> *const u16 {
        self.graph.get_vertex_indices_ptr()
    }

    pub fn translate_offset_by_pixels(&mut self, x: f32, y: f32) {
        self.graph.translate_offset_by_pixels(x, y);
    }

    pub fn count_edges(&self) -> usize {
        self.graph.count_edges()
    }
}

pub struct GraphLayout {
    node_targets: Vec<Vec<usize>>,
    node_sources: Vec<Vec<usize>>,
    node_locations: geometry::Points,
    loading_node_index: usize,
    edges_loaded: usize,
}

impl GraphLayout {
    pub fn new(node_count: usize, spawn_scale: f32) -> GraphLayout {
        let node_targets = (0..node_count).map(|_| Vec::new()).collect();
        let node_sources = (0..node_count).map(|_| Vec::new()).collect();
        let node_locations = geometry::Points::new_random(node_count, spawn_scale);
        GraphLayout {
            node_targets,
            node_sources,
            node_locations,
            loading_node_index: 0,
            edges_loaded: 0,
        }
    }
    pub fn load_edges(&mut self, chunk_array: Uint8Array) {
        debug!("Started loading edges");
        let chunk_buffer = chunk_array.to_vec();
        let mut numbers = vec![0; chunk_buffer.len() / 2];
        LittleEndian::read_u16_into(&chunk_buffer, &mut numbers);
        debug!("Getting targets for node {}...", self.loading_node_index);
        for &num in numbers.iter() {
            // The MAX acts as a delimiter
            if num == u16::MAX {
                self.loading_node_index += 1;
            } else {
                let target_index = num as usize;
                self.node_targets
                    .get_mut(self.loading_node_index)
                    .unwrap()
                    .push(target_index);
                self.node_sources
                    .get_mut(target_index)
                    .unwrap()
                    .push(self.loading_node_index);
                self.edges_loaded += 1;
            }
        }
    }
}

pub struct GraphDisplay {
    layout: GraphLayout,
    display_width: f32,
    display_height: f32,
    display_scale: f32,
    display_offset: geometry::Vector2,
    clipspace_vertices: Vec<f32>,
    vertex_indices: Vec<u16>,
}

impl GraphDisplay {
    pub fn new(
        layout: GraphLayout,
        display_width: f32,
        display_height: f32,
        display_scale: f32,
    ) -> GraphDisplay {
        let aspect_ratio = display_width / display_height;
        let display_offset = layout.node_locations.get_point(0);
        let clipspace_square_offset = NODE_DISPLAY_SQUARE_WIDTH / 2.0 / display_height;
        let clipspace_vertices = layout
            .node_locations
            .to_clipspace(display_offset, &display_scale, &aspect_ratio)
            .get_data();
        let vertex_indices: Vec<u16> = Vec::new();
        GraphDisplay {
            layout,
            display_width,
            display_height,
            display_scale,
            display_offset,
            clipspace_vertices,
            vertex_indices,
        }
    }

    pub fn translate_offset_by_pixels(&mut self, x: f32, y: f32) {
        let pan_rate = DISPLAY_PAN_RATE / self.display_height;
        let new_x = self.display_offset.x - (x * pan_rate) / self.display_aspect_ratio();

        let new_y = self.display_offset.y + y * pan_rate;
        self.display_offset = geometry::Vector2::new(new_x, new_y);
    }

    pub fn update_display_size(&mut self, display_width: f32, display_height: f32) {
        self.display_width = display_width;
        self.display_height = display_height;
    }

    pub fn update_clipspace_vertices(&mut self) {
        let perf = window().unwrap().performance().unwrap();

        let edges_start = perf.now();
        let edges_count = self.count_edges();
        if edges_count > (self.vertex_indices.len() / 2) {
            self.vertex_indices.resize(edges_count * 2, u16::MAX);
            let mut edge_start_index = 0;
            for (source_index, target_indices) in self.layout.node_targets.iter().enumerate() {
                for target_index in target_indices {
                    self.vertex_indices[edge_start_index] =
                        u16::try_from(source_index).expect("Node index should fit u16");
                    self.vertex_indices[edge_start_index + 1] =
                        u16::try_from(*target_index).expect("Node index should fit u16");
                    edge_start_index += 2;
                }
            }
        }
        let edges_elapsed = perf.now() - edges_start;
        debug!("vertex_indices took {} ms", edges_elapsed);

        let verts_start = perf.now();
        let aspect_ratio = self.display_aspect_ratio();
        self.clipspace_vertices = self
            .layout
            .node_locations
            .to_clipspace(self.display_offset, &self.display_scale, &aspect_ratio)
            .get_data();
        let verts_elapsed = perf.now() - verts_start;
        debug!("clipspace_vertices took {} ms", verts_elapsed);
    }

    pub fn get_vertices_ptr(&self) -> *const f32 {
        self.clipspace_vertices.as_ptr()
    }

    pub fn get_vertex_indices_ptr(&self) -> *const u16 {
        self.vertex_indices.as_ptr()
    }

    pub fn count_edges(&self) -> usize {
        self.layout
            .node_targets
            .iter()
            .map(|targets| targets.len())
            .sum()
    }
}

impl GraphDisplay {
    fn display_aspect_ratio(&self) -> f32 {
        self.display_width / self.display_height
    }
}
