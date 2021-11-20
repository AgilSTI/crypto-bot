#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Element {
  pub  id: i32,
  pub  position_x: i32,
  pub  position_y: i32,
  pub  matching_probability: f32,
  pub  matching_probability_minimal: f32
}

impl Element {
   pub fn new(id: i32, position_x: i32, position_y: i32, matching_probability: f32, matching_probability_minimal: f32) -> Element {
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


    