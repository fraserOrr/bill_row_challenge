use rayon::prelude::*;
use std::fs::File;
use std::vec::Vec;
use std::io::{prelude::*, Error};
use std::collections::{BTreeMap, HashMap};
use std::io::BufReader;
use std::time::{Duration, Instant};
use chashmap::CHashMap;
use std::thread;
//our shitty data struct
struct WeatherData {
  max: f64,
  min: f64,
  sum: f64,
  total: f64,
}
impl std::fmt::Display for WeatherData {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "(max: {}, min: {}, sum: {}, total: {})", self.max, self.min, self.sum, self.total)
  }
}

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let start = Instant::now();
    //file stuff
    let file = File::open("./src/measurements_10m.txt")?;
    let  buf_reader = BufReader::new(file);

    // obs a hash map 
    let mut btreemap: BTreeMap<String, WeatherData> = BTreeMap::new();
    let mut hashmap: HashMap<String, WeatherData> = HashMap::new();

    for line in buf_reader.lines() {
 
      line_process(line,&mut hashmap);

      

    }
 
    
    for(k,v) in hashmap.into_iter(){
      let new_data = WeatherData{
        max: v.max,
        min: v.min,
        sum: v.sum,
        total: v.total,
      };
      btreemap.insert(k.to_string(), new_data);
    }
    /* */
    /* 
    hashmap.par_iter().for_each(|(k,v)| {
        let new_data = WeatherData{
          max: v.max,
          min: v.min,
          sum: v.sum,
          total: v.total,
        };
        btreemap.insert(k.to_string(), new_data);
      } 
    );*/
    for (k,v) in btreemap.iter(){
      let avg = v.sum / v.total;
      println!("Station: {}, min {}, avg {:.1}, max {}",k,v.min, avg,v.max);
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
  Ok(())
}

fn line_process(line: Result<String, Error>, hashmap: &mut HashMap<String, WeatherData>){

  let content: Vec<String> = line.expect("something there").split(";").map(|m| m.to_string()).collect();
  let name: String = content[0].clone();
  let value: f64 = content[1].parse::<f64>().expect("convert str to int failed");
  

  //handle_data(&mut btreemap, name, value)
  handle_data3( hashmap, name, value)


}

fn handle_data3(hashmap: &mut HashMap<String, WeatherData>, name: String, value: f64){
  let curr_data = hashmap.get_mut(&name);
  
  match curr_data{
    Some(data_there) =>{
      let mut curr_data =  data_there;
      if curr_data.max < value {
        curr_data.max = value
      }else if curr_data.min > value {
        curr_data.min = value 
      }
    
      curr_data.sum = curr_data.sum + value;
      curr_data.total += 1.0;


    },
    None =>{
      let tmp_data = WeatherData{
        max: value,
        min: value,
        sum: value,
        total: 1.0,
      };
  
      hashmap.insert(name, tmp_data);
    } ,
    }
  //adjust min or max if needed


  
}

// think we are going to do functions early so we can add some aync later maybe lol :(
fn handle_data2(btreemap: &mut BTreeMap<String, WeatherData>, name: String, value: f64){
  let curr_data = btreemap.get_mut(&name);
  
  match curr_data{
    Some(data_there) =>{
      let curr_data =  data_there;
      if curr_data.max < value {
        curr_data.max = value
      }else if curr_data.min > value {
        curr_data.min = value 
      }
    
      curr_data.sum = curr_data.sum + value;
      curr_data.total += 1.0;


    },
    None =>{
      let tmp_data = WeatherData{
        max: value,
        min: value,
        sum: value,
        total: 1.0,
      };
  
      btreemap.insert(name, tmp_data);
    } ,
    }
  //adjust min or max if needed


  
}

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
