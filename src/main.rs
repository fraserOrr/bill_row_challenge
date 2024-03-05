use std::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
use std::collections::btree_map::Values;
use std::fs::File;
use std::iter;
use std::vec::Vec;
use std::io::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::io::BufReader;

//our shitty data struct
struct WeatherData {
  max: f64,
  min: f64,
  sum: f64,
  total: f64,
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    //file stuff
    let file = File::open("./src/measurements_10m.txt")?;
    let  buf_reader = BufReader::new(file);

    // obs a hash map 
    let mut btreemap: BTreeMap<String, WeatherData> = BTreeMap::new();


    for line in buf_reader.lines() {
        
      let content: Vec<String> = line.expect("something there").split(";").map(|m| m.to_string()).collect();
      let name: String = content[0].clone();
      let value: f64 = content[1].parse::<f64>().expect("convert str to int failed");
      

      handle_data(&mut btreemap, name, value)

    }
 
    
    
    for (k,v) in btreemap.iter(){
      let avg = v.sum / v.total;
      println!("Station: {}, min {}, avg {:.1}, max {}",k,v.min, avg,v.max);
    }

  Ok(())
}




// think we are going to do functions early so we can add some aync later maybe lol :(

fn handle_data(btreemap: &mut BTreeMap<String, WeatherData>, name: String, value: f64){
  if btreemap.contains_key(&name)==false{
    let tmp_data = WeatherData{
      max: value,
      min: value,
      sum: value,
      total: 1.0,
    };

    btreemap.insert(name, tmp_data);
  }else{
    let mut curr_data = btreemap.get_mut(&name).expect("no ref found to name");
    //adjust min or max if needed
    if curr_data.max < value {
      curr_data.max = value
    }else if curr_data.min > value {
      curr_data.min = value 
    }

    curr_data.sum = curr_data.sum + value;
    curr_data.total += 1.0;

  }
}
