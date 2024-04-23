#[allow(dead_code)]
#[derive(Debug)]
pub struct Employee {
  pub name: String,
  pub gender: String,
  pub age: u32, 
  pub nums: [i32; 6]
}

impl Employee {
  pub fn fn_details(&self) -> String {
    format!("name: {}, Gender: {}, age: {}", &self.name, &self.gender, &self.age)
  }
    
}