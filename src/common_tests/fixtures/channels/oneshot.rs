use rstest::{ fixture };
use tokio::sync::oneshot::{ Sender, Receiver };

#[fixture]
pub fn get_one_shot_channel<T>() -> (Sender<T>, Receiver<T>) {
    tokio::sync::oneshot::channel::<T>()
}