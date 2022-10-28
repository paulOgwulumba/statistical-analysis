//! # measures_of_dispersion
//! This module contains utilities for performing measures of 
//! central tendency on numerical data sets.

/// This calculates the range of a numerical data set.
/// A range is defined as the difference between the highest value in the
/// data set and the lowest value of the data set.
/// 
/// # Example
/// ```
/// let mut data = vec![1, 3, 5, 6];
/// let range = measures_of_dispersion::get_range(data);
/// ```
pub fn get_range(mut _data: Vec<i32>) -> Option<i32> {
  match _data.len() {
    0 => None,
    _ => {
      _data.sort();
      let min_val = _data[0];
      let max_val = _data[_data.len() - 1];
      Some(max_val - min_val)
    }
  }
}

/// This calculates the interquartile range of a numerical data set.
pub fn get_interquartile_range(mut _data: Vec<i32>) -> Option<f32> {
  match _data.len() {
    0 => None,
    _ => {
      use crate::measures_of_central_tendency::get_median;
      _data.sort();
      let (quartile_1_list, quartile_3_list): (Vec<i32>, Vec<i32>) = match _data.len() % 2 {
        0 => {
          let middle = _data.len() / 2;
          let mut vector_1 = Vec::<i32>::new();
          let mut vector_2 = Vec::<i32>::new();

          for num in 0..middle {
            vector_1.push(_data[num]);
          }

          for num in middle.._data.len() {
            vector_2.push(_data[num]);
          }

          (vector_1, vector_2)
        }
        _ => {
          let middle = _data.len() / 2;
          let mut vector_1 = Vec::<i32>::new();
          let mut vector_2 = Vec::<i32>::new();

          for num in 0..middle {
            vector_1.push(_data[num]);
          }

          for num in (middle + 1)..(_data.len()) {
            vector_2.push(_data[num]);
          }

          (vector_1, vector_2)
        }
      };
      let quartile_1 = get_median(quartile_1_list).unwrap_or_default();
      let quartile_3 = get_median(quartile_3_list).unwrap_or_default();

      Some(quartile_3 - quartile_1)
    }
  }
}

pub fn get_standard_deviation(mut _data: Vec<i32>) -> Option<f32> {
  match _data.len() {
    0 => None,
    _ => Some(
      crate::measures_of_dispersion::get_variance(_data)
        .unwrap_or_default()
        .sqrt(),
    ),
  }
}

pub fn get_variance(mut _data: Vec<i32>) -> Option<f32> {
  match _data.len() {
    0 => None,
    _ => {
      use crate::measures_of_central_tendency::get_mean;
      let mean: f32 = get_mean(_data.clone()).unwrap_or_default();

      let mut modified_data = Vec::<f32>::new();
      for num in _data.iter() {
        modified_data.push((*num as f32 - mean).powi(2));
      }

      let mut sum_of_modified_data: f32 = 0.0;
      for num in modified_data.iter() {
        sum_of_modified_data += num;
      }

      Some(sum_of_modified_data / modified_data.len() as f32)
    }
  }
}
