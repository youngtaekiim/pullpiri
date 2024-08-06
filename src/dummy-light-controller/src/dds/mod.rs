pub mod sender;
pub mod receiver;

pub trait EventListener {
    async fn run(&self);
}
