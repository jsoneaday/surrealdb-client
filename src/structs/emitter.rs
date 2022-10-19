use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{ Context, Poll };
use std::time::{ Duration, Instant };
use std::hash::Hash;
use tokio::sync::{ oneshot};

pub async fn once<EN, V>(emitter: &Emitter<EN, V>, event_name: &EN) -> V
where
    EN: Hash + Eq,
    V: Copy + 'static
{
    let (tx, rx) = oneshot::channel();
    
    emitter.once(event_name, Box::new(|emitter, args| {
        tx.send(args);
    }));

    rx.await.unwrap()
}

struct Emitter<K, V> 
where
    K: Hash + Eq
{
    when: Instant,
    events: HashMap<K, Vec<Box<dyn FnOnce(&Self, Vec<V>) -> ()>>>
}

impl<K, V> Emitter<K, V>
where
    K: Hash + Eq ,
    V: Copy + 'static
{
    fn new() -> Self {
        Emitter {
            when: Instant::now() + Duration::from_millis(10),
            events: HashMap::new()
        }
    }

    async fn next_event(&self, event_name: &K) -> V {
		once(self, event_name).await
	}
    
    fn on(
        &mut self,
		event_name: &K,
		listener: Box<dyn FnOnce(&Self, Vec<V>) -> ()>
	) -> Self {
		if let None = self.events.get(event_name) {
			self.events.insert(*event_name.clone(), vec![]);
		}
		self.events.get(event_name).unwrap().insert(0, listener);
		
        *self
	}

    fn once(
        &self,
		event_name: &K,
		listener: Box<dyn FnOnce(&Self, Vec<V>) -> ()> 
	) -> Self {
		self.on(event_name, Box::new(|listener_self, args| {
			listener(self, args);
		}));

        *self
	}

    fn emit(&self, event_name: &K, args: Vec<V>) -> Self {
        if let Some(listeners) = self.events.get(event_name) {
            for listener in listeners {
			    listener(self, args);
            }
        }		

		*self
	}
}

impl<K, V> Future for Emitter<K, V>
where
    K: Hash + Eq
{
    type Output = V;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {

            Poll::Ready(self.events[""].into())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn testerguy() {
    Emitter{
        when: Instant::now() + Duration::from_millis(10),
        events: HashMap::new()
    }.await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {

    }
}