use std::collections::HashMap;

pub fn get_mean(data: Vec<i32>) -> Option<f32> {
  match data.len() {
    0 => None,
    _ => {
      let mut sum: i32 = 0;
      for num in data.iter() {
        sum += num;
      }
      Some(sum as f32 / data.len() as f32)
    }
  }
}

pub fn get_mode(mut _data: Vec<i32>) -> Vec<i32> {
  _data.sort();
  let mut map: HashMap<_, _> = HashMap::new();
  for num in _data.iter() {
    let count = map.entry(num).or_insert(0);
    *count += 1;
  }

  let mut max_count = 0;

  for (_key, value) in &map {
    if value > &max_count {
      max_count = *value;
    }
  }

  let mut vector: Vec<i32> = Vec::new();

  for (key, value) in map {
    if value == max_count {
      vector.push(*key);
    }
  }

  vector
}

pub fn get_median(mut _data: Vec<i32>) -> Option<f32> {
  match _data.len() {
    0 => None,
    _ => {
      _data.sort();
      match _data.len() % 2 {
        0 => {
          let middle_index_1 = _data.len() / 2 - 1;
          let middle_index_2 = middle_index_1 + 1;
          let mut middle_indices = Vec::<i32>::new();

          middle_indices.push(_data[middle_index_1]);
          middle_indices.push(_data[middle_index_2]);

          crate::measures_of_central_tendency::get_mean(middle_indices)
        }
        _ => {
          let middle_index = _data.len() / 2;
          match _data.get(middle_index) {
            Some(val) => Some(*val as f32),
            None => None,
          }
        }
      }
    }
  }
}
