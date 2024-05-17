use byteorder::{ByteOrder, LittleEndian};

use js_sys;
use log::{debug, Level};
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
extern crate console_error_panic_hook;
use std::panic;

mod geometry;
mod input;

const DISPLAY_PAN_RATE: f32 = 1.0;
const AUTOPAN_RATE_MUL: f32 = 512.0;
const DISPLAY_ZOOM_RATE: f32 = 1.25;

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
        node_locations: js_sys::Float32Array,
        display_width: f32,
        display_height: f32,
        display_scale: f32,
    ) -> GraphFacade {
        let layout = GraphLayout::new(node_count, node_locations);
        let display = GraphDisplay::new(layout, display_width, display_height, display_scale);
        GraphFacade { graph: display }
    }

    pub fn load_edges(&mut self, chunk_array: js_sys::Uint8Array) {
        self.graph.layout.load_edges(chunk_array);
    }

    pub fn update_display_size(&mut self, display_width: f32, display_height: f32) {
        self.graph
            .update_display_size(display_width, display_height);
    }
    pub fn update_edges(&mut self) {
        self.graph.update_edges()
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

    pub fn pan(&mut self, x: f32, y: f32) {
        self.graph.pan(x, y);
    }

    pub fn zoom_in(&mut self) {
        self.graph.zoom_in();
    }

    pub fn zoom_out(&mut self) {
        self.graph.zoom_out();
    }

    pub fn touch_start(&mut self, touch: input::TouchSet) {
        self.graph.touch_start(touch);
    }

    pub fn touch_move(&mut self, touch: input::TouchSet) {
        self.graph.touch_move(touch);
    }

    pub fn get_visible_node_page_locations(&self) -> Result<JsValue, JsValue> {
        self.graph.get_visible_node_page_locations()
    }

    pub fn autopan(&mut self, node_id: usize) {
        self.graph.autopan(node_id);
    }
}

pub struct GraphLayout {
    node_targets: Vec<Vec<usize>>,
    node_sources: Vec<Vec<usize>>,
    node_locations: geometry::Points,
    loading_node_index: usize,
    edges_loaded: usize,
    loading_remainder: Option<u8>,
}

impl GraphLayout {
    pub fn new(node_count: usize, locations: js_sys::Float32Array) -> GraphLayout {
        let node_targets = (0..node_count).map(|_| Vec::new()).collect();
        let node_sources = (0..node_count).map(|_| Vec::new()).collect();
        let node_locations = geometry::Points::new(locations.to_vec());
        GraphLayout {
            node_targets,
            node_sources,
            node_locations,
            loading_node_index: 0,
            edges_loaded: 0,
            loading_remainder: None,
        }
    }
    pub fn load_edges(&mut self, chunk_array: js_sys::Uint8Array) {
        let mut chunk_buffer = chunk_array.to_vec();

        if let Some(leftover) = self.loading_remainder {
            chunk_buffer.insert(0, leftover);
            self.loading_remainder = None;
        }
        if chunk_buffer.len() % 2 == 1 {
            self.loading_remainder = chunk_buffer.pop();
        }

        let mut numbers = vec![0; chunk_buffer.len() / 2];
        LittleEndian::read_u16_into(&chunk_buffer, &mut numbers);
        debug!("Getting targets for node {}...", self.loading_node_index);
        for &num in numbers.iter() {
            // The MAX acts as a delimiter
            if num == u16::MAX {
                self.loading_node_index += 1;
            } else {
                let target_index = num as usize;

                if self.node_targets.len() < target_index {
                    debug!("usize target index: {}", num);
                    debug!("u16 target index: {}", num);
                }

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
    current_touches: Option<Vec<input::TouchSet>>,
    prev_touch: Option<input::TouchSet>,
    clipspace_locations: geometry::Points,
    clipspace_vertices: Vec<f32>,
    vertex_indices: Vec<u16>,
    autopanning: bool,
    autopan_dest: geometry::Vector2,
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
        let prev_touch = None;
        let current_touches = None;
        let clipspace_locations =
            layout
                .node_locations
                .to_clipspace(display_offset, &display_scale, &aspect_ratio);
        let clipspace_vertices = clipspace_locations.get_data();
        let vertex_indices: Vec<u16> = Vec::new();
        let autopanning = false;
        let autopan_dest = display_offset;
        GraphDisplay {
            layout,
            display_width,
            display_height,
            display_scale,
            display_offset,
            current_touches,
            prev_touch,
            clipspace_locations,
            clipspace_vertices,
            vertex_indices,
            autopanning,
            autopan_dest,
        }
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

    pub fn get_visible_node_page_locations(&self) -> Result<JsValue, JsValue> {
        let top_right_x = 1.0 - (100.0 / self.display_width * 2.0);
        let bottom_left_y = -1.0 + (35.0 / self.display_width * 2.0);
        let text_bounds: geometry::Rect = geometry::Rect {
            bottom_left: geometry::Vector2 {
                x: -1.0,
                y: bottom_left_y,
            },
            top_right: geometry::Vector2 {
                x: top_right_x,
                y: 1.0,
            },
        };
        let mut locations: HashMap<usize, geometry::Vector2> = HashMap::new();
        for (node_id, loc) in self.clipspace_locations.iter().enumerate() {
            if text_bounds.contains(loc) {
                let x = ((loc.x + 1.0) / 2.0) * self.display_width;
                let y = self.display_height - (((loc.y + 1.0) / 2.0) * self.display_height);
                locations.insert(node_id, geometry::Vector2 { x, y });
            }
        }
        Ok(serde_wasm_bindgen::to_value(&locations)?)
    }

    pub fn count_edges(&self) -> usize {
        self.layout
            .node_targets
            .iter()
            .map(|targets| targets.len())
            .sum()
    }

    pub fn update_edges(&mut self) {
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
    }

    pub fn update_display_size(&mut self, display_width: f32, display_height: f32) {
        self.display_width = display_width;
        self.display_height = display_height;
    }

    pub fn update_clipspace_vertices(&mut self) {
        self.update_display();
        let aspect_ratio = self.get_aspect_ratio();
        self.clipspace_locations = self.layout.node_locations.to_clipspace(
            self.display_offset,
            &self.display_scale,
            &aspect_ratio,
        );
        self.clipspace_vertices = self.clipspace_locations.get_data();
    }

    pub fn pan(&mut self, x: f32, y: f32) {
        self.autopanning = false;
        let pan_rate = self.get_pan_rate();
        self.display_offset.x += x * pan_rate;
        self.display_offset.y += y * pan_rate;
    }

    pub fn zoom_in(&mut self) {
        self.display_scale *= DISPLAY_ZOOM_RATE;
    }

    pub fn zoom_out(&mut self) {
        self.display_scale /= DISPLAY_ZOOM_RATE;
    }

    pub fn touch_start(&mut self, touch: input::TouchSet) {
        self.autopanning = false;
        self.prev_touch = None;
        self.current_touches = Some(vec![touch]);
    }

    pub fn touch_move(&mut self, touch: input::TouchSet) {
        self.current_touches
            .as_mut()
            .expect("current_touches should exist on move")
            .push(touch);
    }

    fn update_display(&mut self) {
        if self.autopanning {
            //let autopan_rate = self.get_pan_rate() * self.display_scale * AUTOPAN_RATE_MUL;
            let autopan_rate = self.get_pan_rate() * AUTOPAN_RATE_MUL;
            let diff = self.autopan_dest - self.display_offset;
            if diff.magnitude() <= autopan_rate {
                self.display_offset = self.autopan_dest;
                self.autopanning = false;
            } else {
                let direction = diff
                    .unit()
                    .expect("Diff vector should have non-zero magnitude");
                self.display_offset += direction * autopan_rate;
            }
        }
        if let Some(current_touches) = &self.current_touches {
            let mut touches = current_touches.clone();
            if let Some(prev_touch) = &self.prev_touch {
                touches.insert(0, prev_touch.clone());
            }

            if touches.len() < 2 {
                return;
            }

            let pinch: f32 = touches.as_slice().windows(2).map(input::pinch_diff).sum();
            if pinch > 0.0 {
                self.zoom_in()
            } else if pinch < 0.0 {
                self.zoom_out()
            }
            let offset_addend: geometry::Vector2 =
                touches.as_slice().windows(2).map(input::touch_offset).sum();
            self.display_offset += (offset_addend.flip_y() * self.get_pan_rate());

            self.prev_touch = touches.pop();
            self.current_touches = Some(Vec::new());
        }
    }

    fn get_aspect_ratio(&self) -> f32 {
        self.display_width / self.display_height
    }

    fn get_pan_rate(&self) -> f32 {
        ((DISPLAY_PAN_RATE * 2.0) / self.display_scale) / self.display_height
    }

    pub fn autopan(&mut self, node_id: usize) {
        self.autopan_dest = self.layout.node_locations.get_point(node_id);
        self.autopanning = true
    }

    pub fn autopan_in_progress(&self) -> bool {
        self.autopanning
    }
}
