use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}, thread, time::Duration};

#[derive(Debug)]
struct Cache{
     store:RwLock<HashMap<String,i32>>
}

impl Cache{
    fn new()->Self{
       Self { store: RwLock::new(HashMap::new()) }
    }

    fn insert(&self,name :String,value:i32){
       println!("Started Insertinggg");
       thread::sleep(Duration::from_millis(50));      
       let mut map = self.store.write().unwrap();
       map.insert(name, value);
       println!("Inserted");
    }

    fn get(&self,name:String)->Option<i32>{
        println!("Started Reading");
        thread::sleep(Duration::from_millis(50)); 
        let map = self.store.read().unwrap();
        println!("reading completed");
        map.get(&name).copied()
    }
}


fn main() {
   let cache=Arc::new(Cache::new());
   cache.insert("apple".to_string(), 5);
   cache.insert("mango".to_string(), 6);
   let mut handles= vec![];
   let names=vec!["hello".to_string(),"hii".to_string(),"bye".to_string(),"wah".to_string()];
   for i in 0..4{
       let cache_clone =Arc::clone(&cache);
       let handle = thread::spawn(move ||{
             if let Some(val) = cache_clone.get("apple".to_string()){
                 println!("reader-->{}--->got {}",i,val);
             }
       });
       handles.push(handle);
   }

       let cache_clone =Arc::clone(&cache);
       let handle = thread::spawn(move ||{
             cache_clone.insert("apple".to_string(),6);
       });
       handles.push(handle);
   

   for i in 5..10{
       let cache_clone =Arc::clone(&cache);
       let handle = thread::spawn(move ||{
             if let Some(val) = cache_clone.get("apple".to_string()){
                 println!("reader-->{}--->got {}",i,val);
             }
       });
       handles.push(handle);
   }
   for handle in handles{
      handle.join().unwrap();
   }
}
