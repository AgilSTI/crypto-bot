#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Element {
  pub  id: u32,
  pub  position_x: u32,
  pub  position_y: u32,
  pub  matching_probability: f32,
  pub  matching_probability_minimal: f32
}

impl Element {
   pub fn new(id: u32, position_x: u32, position_y: u32, matching_probability: f32, matching_probability_minimal: f32) -> Element {
        Element {
            id,
            position_x,
            position_y,
            matching_probability,
            matching_probability_minimal,
        }
    }

    pub fn to_owned(&self) -> &Element {
        self
        }
    }