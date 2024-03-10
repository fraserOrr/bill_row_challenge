
use std::vec::Vec;
use std::io::{prelude::*, Error};
use std::collections::{BTreeMap, HashMap};
use tokio::sync::{mpsc, oneshot};
use std::time::{Duration, Instant};




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
const THREAD_COUNT: usize = 1;


async fn rec_data(thread_rx: async_channel::Receiver<(String, f64)>, thread_id: usize) -> HashMap<String, WeatherData>{
  println!("receiver started: {}", thread_id);
  let mut hashmap: HashMap<String, WeatherData> = HashMap::new();
  
  while let Ok(message) =  thread_rx.recv().await {
    //print!("Rec : {},{}", message.0,message.1);
    handle_data3(&mut hashmap, message.0, message.1);
    
  }
  //print_results(&hashmap);
  return hashmap;
  
}

fn handle_data3(hashmap: &mut HashMap<String, WeatherData>, name: String, value: f64){
  let curr_data = hashmap.get_mut(&name);
  
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
  
      hashmap.insert(name, tmp_data);
    } ,
    }
  //adjust min or max if needed


  
}


fn print_results(hashmap: &HashMap<String, WeatherData> ){
    let mut btreemap: BTreeMap<String, WeatherData> = BTreeMap::new();

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
}


async fn read_data(thread_tx: async_channel::Sender<(String, f64)>){
    
  thread_tx.downgrade();
    //user current thread to send ? or should i spawn another thread to read and send?
    //let (task_tx, mut task_rx) = tokio::sync::mpsc::channel::<(String, f64)>(1000);
  println!("reader started");
  let file = std::fs::File::open("./src/measurements_10m.txt").expect("no file");
  let  buf_reader = std::io::BufReader::new(file);

  //
    for line in buf_reader.lines(){
      
      let content: Vec<String> = line.expect("bad line").split(";").map(|m| m.to_string()).collect();
      let name: String = content[0].clone();
      let value: f64 = content[1].parse::<f64>().expect("convert str to int failed");
      let payload: (String, f64) = (name,value);
        
      let _ = thread_tx.send(payload).await;

    }
  //});

}





#[tokio::main]
async fn main(){
  let start = Instant::now();
  // objects
  // obs a hash map 
    
    

    
  let (thread_tx, thread_rx) = async_channel::unbounded();;
  thread_tx.downgrade();

  let mut handles: Vec<tokio::task::JoinHandle<(HashMap<String, WeatherData>)>> = vec![];
  for thread_id in 0..THREAD_COUNT {
    let rx_clone = thread_rx.clone();
    let handle = tokio::spawn(async move {
      return rec_data(rx_clone, thread_id).await;
    });
    handles.push(handle);
  }
  /*  */
  //let tokio_handle = Handle::current();
  //start receiving thread here?
  tokio::spawn(async move {
    read_data(thread_tx).await;
  }); 

 
  

  //recieve task
 

    
  let duration = start.elapsed();
  println!("Time elapsed in process is: {:?}", duration);

 
  for handle in handles {
    let hashmap = handle.await.expect("Thread join error");
    print_results(&hashmap);
      
  }


  
  

  let duration = start.elapsed();
  println!("Time elapsed end format is: {:?}", duration);
  
   
  
}


