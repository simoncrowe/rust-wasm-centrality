use byteorder::{ByteOrder, LittleEndian};

use js_sys::{Float32Array, Uint8Array};
use log::{debug, Level};
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
extern crate console_error_panic_hook;
use std::panic;

mod geometry;

const DISPLAY_PAN_RATE: f32 = 1.0;
const DISPLAY_ZOOM_RATE: f32 = 1.25;
const CLIPSPACE_BOUNDS: geometry::Rect = geometry::Rect {
    bottom_left: geometry::Vector2 { x: -1.0, y: -1.0 },
    top_right: geometry::Vector2 { x: 1.0, y: 1.0 },
};

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

#[derive(Serialize)]
pub struct NodeLocation {
    node_id: usize,
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl GraphFacade {
    pub fn new(
        node_count: usize,
        node_locations: Float32Array,
        display_width: f32,
        display_height: f32,
        display_scale: f32,
    ) -> GraphFacade {
        let layout = GraphLayout::new(node_count, node_locations);
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

    pub fn get_vertex_indices_len(&self) -> usize {
        self.graph.get_vertex_indices_len()
    }

    pub fn pan_touch(&mut self, x: f32, y: f32) {
        self.graph.pan_touch(x, y);
    }

    pub fn pan(&mut self, x: f32, y: f32) {
        self.graph.pan(x, y);
    }

    pub fn zoom_in(&mut self) {
        self.graph.zoom_in();
    }

    pub fn zoom_out(&mut self) {
        self.graph.zoom_out();
    }

    pub fn get_visible_node_page_locations(&self) -> Result<JsValue, JsValue> {
        self.graph.get_visible_node_page_locations()
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
    pub fn new(node_count: usize, locations: Float32Array) -> GraphLayout {
        let node_targets = (0..node_count).map(|_| Vec::new()).collect();
        let node_sources = (0..node_count).map(|_| Vec::new()).collect();
        let node_locations = geometry::Points::new(locations.to_vec());
        GraphLayout {
            node_targets,
            node_sources,
            node_locations,
            loading_node_index: 0,
            edges_loaded: 0,
        }
    }
    pub fn load_edges(&mut self, chunk_array: Uint8Array) {
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
    pan_target: geometry::Vector2,
    pan_actual: geometry::Vector2,
    clipspace_locations: geometry::Points,
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
        let clipspace_locations =
            layout
                .node_locations
                .to_clipspace(display_offset, &display_scale, &aspect_ratio);
        let clipspace_vertices = clipspace_locations.get_data();
        let vertex_indices: Vec<u16> = Vec::new();
        let pan_target = geometry::Vector2 { x: 0.0, y: 0.0 };
        let pan_actual = geometry::Vector2 { x: 0.0, y: 0.0 };
        GraphDisplay {
            layout,
            display_width,
            display_height,
            display_scale,
            display_offset,
            pan_target,
            pan_actual,
            clipspace_locations,
            clipspace_vertices,
            vertex_indices,
        }
    }

    pub fn update_display_size(&mut self, display_width: f32, display_height: f32) {
        self.display_width = display_width;
        self.display_height = display_height;
    }

    pub fn pan_touch(&mut self, x: f32, y: f32) {
        let pan_rate = self.get_pan_rate();
        debug!("Inputs - x: {}, y: {}", x, y);
        self.pan_target.x += x * pan_rate;
        self.pan_target.y += y * pan_rate;
    }

    pub fn pan(&mut self, x: f32, y: f32) {
        let pan_rate = self.get_pan_rate();
        debug!("Inputs - x: {}, y: {}", x, y);
        self.display_offset.x += x * pan_rate;
        self.display_offset.y += y * pan_rate;
    }

    pub fn zoom_in(&mut self) {
        self.display_scale *= DISPLAY_ZOOM_RATE;
    }

    pub fn zoom_out(&mut self) {
        self.display_scale /= DISPLAY_ZOOM_RATE;
    }

    pub fn update_clipspace_vertices(&mut self) {
        // Ensure that the indices used for drawing edges are up-to-date
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

        self.update_pan();

        let aspect_ratio = self.get_aspect_ratio();
        self.clipspace_locations = self.layout.node_locations.to_clipspace(
            self.display_offset,
            &self.display_scale,
            &aspect_ratio,
        );
        self.clipspace_vertices = self.clipspace_locations.get_data();
    }

    pub fn get_visible_node_page_locations(&self) -> Result<JsValue, JsValue> {
        let mut locations: HashMap<usize, geometry::Vector2> = HashMap::new();
        for (node_id, loc) in self.clipspace_locations.iter().enumerate() {
            if CLIPSPACE_BOUNDS.contains(loc) {
                let x = ((loc.x + 1.0) / 2.0) * self.display_width;
                let y = self.display_height - (((loc.y + 1.0) / 2.0) * self.display_height);
                locations.insert(node_id, geometry::Vector2 { x, y });
            }
        }
        Ok(serde_wasm_bindgen::to_value(&locations)?)
    }

    pub fn get_vertices_ptr(&self) -> *const f32 {
        self.clipspace_vertices.as_ptr()
    }

    pub fn get_vertex_indices_ptr(&self) -> *const u16 {
        self.vertex_indices.as_ptr()
    }

    pub fn get_vertex_indices_len(&self) -> usize {
        self.vertex_indices.len()
    }
}

impl GraphDisplay {
    fn get_aspect_ratio(&self) -> f32 {
        self.display_width / self.display_height
    }

    pub fn count_edges(&self) -> usize {
        self.layout
            .node_targets
            .iter()
            .map(|targets| targets.len())
            .sum()
    }

    fn get_pan_rate(&self) -> f32 {
        ((DISPLAY_PAN_RATE * 2.0) / self.display_scale) / self.display_height
    }

    fn update_pan(&mut self) {
        let pan_rate = self.get_pan_rate();
        debug!("Pan rate: {}", pan_rate);
        let pan_target_unit = self.pan_target.unit();
        debug!("Pan target: {:?}", self.pan_target);
        debug!("Pan target mag: {:?}", self.pan_target.magnitude());
        debug!("Pan target unit: {:?}", pan_target_unit);
        match pan_target_unit {
            Some(pan_target_unit) => {
                self.pan_actual += self.pan_target * pan_rate;
                if self.pan_target.magnitude() > pan_rate {
                    self.pan_target -= pan_target_unit * pan_rate;
                } else {
                    self.pan_target = geometry::Vector2 { x: 0.0, y: 0.0 };
                }
            }
            None => {
                let pan_actual_unit = self.pan_actual.unit();
                match pan_actual_unit {
                    Some(pan_actual_unit) => {
                        if self.pan_actual.magnitude() > pan_rate {
                            self.pan_actual -= pan_actual_unit * pan_rate;
                        } else {
                            self.pan_actual = geometry::Vector2 { x: 0.0, y: 0.0 };
                        }
                    }
                    None => {}
                }
            }
        }

        self.display_offset += self.pan_actual;
    }
}
