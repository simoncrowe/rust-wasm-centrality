use wasm_bindgen::prelude::*;

pub fn random_location(scale_x: f32, scale_y: f32) -> Vec<f32> {
    let x_loc = js_sys::Math::random() as f32 * scale_x;
    let y_loc = js_sys::Math::random() as f32 * scale_y;
    vec![x_loc, y_loc]
}

#[wasm_bindgen]
pub struct Rect {
    bottom_left: Vec<f32>,
    top_right: Vec<f32>,
}

#[wasm_bindgen]
impl Rect {
    pub fn new(bottom_left: Vec<f32>, top_right: Vec<f32>) -> Rect {
        Rect {
            bottom_left,
            top_right,
        }
    }
}

impl Rect {
    pub fn contains(&self, point: &Vec<f32>) -> bool {
        point[0] > self.bottom_left[0]
            && point[0] < self.top_right[0]
            && point[1] > self.bottom_left[1]
            && point[1] < self.top_right[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_contains_point_inside() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_inside = vec![1.0, 1.0];

        assert_eq!(true, rect.contains(&point_inside));
    }

    #[test]
    fn test_rect_contains_point_low_left() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_low_left = vec![-1.0, -1.0];

        assert_eq!(false, rect.contains(&point_low_left));
    }

    #[test]
    fn test_rect_contains_point_low() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_low = vec![3.0, -1.0];

        assert_eq!(false, rect.contains(&point_low));
    }

    #[test]
    fn test_rect_contains_point_low_right() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_low_right = vec![5.7, -1.0];

        assert_eq!(false, rect.contains(&point_low_right));
    }

    #[test]
    fn test_rect_contains_point_right() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_right = vec![5.7, 1.0];

        assert_eq!(false, rect.contains(&point_right));
    }

    #[test]
    fn test_rect_contains_point_high_right() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_high_right = vec![7.1, 5.0];

        assert_eq!(false, rect.contains(&point_high_right));
    }

    #[test]
    fn test_rect_contains_point_high() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_high = vec![2.7, 5.0];

        assert_eq!(false, rect.contains(&point_high));
    }

    #[test]
    fn test_rect_contains_point_high_left() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_high_right = vec![-2.3, 5.1];

        assert_eq!(false, rect.contains(&point_high_right));
    }

    #[test]
    fn test_rect_contains_point_left() {
        let bottom_left = vec![0.0, 0.0];
        let top_right = vec![5.5, 4.5];
        let rect = Rect::new(bottom_left, top_right);

        let point_left = vec![-2.3, 2.5];

        assert_eq!(false, rect.contains(&point_left));
    }
}
