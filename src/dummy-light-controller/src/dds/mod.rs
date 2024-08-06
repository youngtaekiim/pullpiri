pub mod receiver;
pub mod sender;

pub trait EventListener {
    async fn run(&self);
}
