use std::collections::HashMap;
use std::ops::DerefMut;
use std::time::{ Duration, Instant };
use std::hash::Hash;
use tokio::sync::{ oneshot};


pub async fn once<EN, V>(emitter: &Emitter<EN, V>, event_name: &EN) -> Vec<V>
where
    EN: Hash + Eq + Copy,
    V: Copy + ?Sized + DerefMut + 'static
{
    let (tx, rx) = oneshot::channel();

    emitter.once(event_name, Box::new(|_, args| {                
        tx.send(args.iter().map(|arg| {
            arg.clone()
        }).collect());
    }));

    rx.await.unwrap()
}

struct Emitter<K, V> 
where
    K: Hash + Eq + Copy,
    V: Copy + 'static
{
    when: Instant,
    events: HashMap<K, Vec<Box<dyn FnOnce(&Self, Vec<V>) -> ()>>>
}

impl<K, V> Emitter<K, V>
where
    K: Hash + Eq + Copy,
    V: Copy + 'static
{
    fn new() -> Self {
        Emitter {
            when: Instant::now() + Duration::from_millis(10),
            events: HashMap::new()
        }
    }

    async fn next_event(&mut self, event_name: &K) -> Vec<V> {
		once(self, event_name).await
	}
    
    fn on(
        mut self,
		event_name: &K,
		listener: Box<dyn FnOnce(&Self, Vec<V>) -> ()>
	) -> Self {
		if let None = self.events.get(event_name) {
			self.events.insert(event_name.clone(), vec![]);
		}
		self.events.get_mut(event_name).unwrap().insert(0, listener);
		
        self
	}

    fn once(
        mut self,
		event_name: &K,
		listener: Box<dyn FnOnce(&Self, Vec<V>) -> ()> 
	) -> Self {
		self.on(event_name, Box::new(|listener_self, args| {
			listener(&self, args);
		}));

        self
	}

    fn emit(self, event_name: &K, args: Vec<V>) -> Self {
        if let Some(listeners) = self.events.get_mut(event_name) {
            for listener in listeners {
			    listener(&self, args);
            }
        }		

		self
	}
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {

    }
}