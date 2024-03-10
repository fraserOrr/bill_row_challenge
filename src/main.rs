use rayon::prelude::*;
use std::fs::File;
use std::vec::Vec;
use std::thread;
use std::io::{prelude::*, Error};
use std::collections::{BTreeMap, HashMap};
use std::io::BufReader;
use std::time::{Duration, Instant};
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
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let start = Instant::now();
    // objects
    // obs a hash map 
    let mut btreemap: BTreeMap<String, WeatherData> = BTreeMap::new();
    
    //file stuff
    
    //thread pool?
    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();

    let (task_tx, mut task_rx) = tokio::sync::mpsc::channel::<(String, f64)>(100);
    let (thread_tx, thread_rx) = std::sync::mpsc::channel::<(String, f64)>();

    
    
  //start receiving thread here?
  thread::spawn( move || {
    let file = File::open("./src/measurements_10m.txt").expect("no file");
    let  buf_reader = BufReader::new(file);
    //user current thread to send ? or should i spawn another thread to read and send?
    for line in buf_reader.lines(){
      let content: Vec<String> = line.expect("something there").split(";").map(|m| m.to_string()).collect();
      let name: String = content[0].clone();
      let value: f64 = content[1].parse::<f64>().expect("convert str to int failed");
      let payload: (String, f64) = (name,value);   
      
      //can we make a lines concurrent

      thread_tx.send(payload).unwrap();
      
    };

  }); 



  
 
  let mut hashmap: HashMap<String, WeatherData> = HashMap::new();

  //recieve task
  let handle = tokio::spawn(async move {
    for message in thread_rx {
      handle_data3(&mut hashmap, message.0, message.1)
      //print!("Rec : {},{}", message.0,message.1);
    }

    return hashmap;
  });

    
  let duration = start.elapsed();
  println!("Time elapsed in process is: {:?}", duration);



  let hashmap = handle.await.unwrap();

  for(k,v) in hashmap.into_iter(){
    let new_data = WeatherData{
      max: v.max,
      min: v.min,
      sum: v.sum,
      total: v.total,
    };
    btreemap.insert(k.to_string(), new_data);
  }
  for (k,v) in btreemap.iter(){
    let avg = v.sum / v.total;
    println!("Station: {}, min {}, avg {:.1}, max {}",k,v.min, avg,v.max);
  }
  let duration = start.elapsed();
  println!("Time elapsed end format is: {:?}", duration);
  Ok(())
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