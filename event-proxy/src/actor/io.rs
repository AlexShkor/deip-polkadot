#[async_trait::async_trait]
pub trait ActorI<I> {
    async fn recv(&mut self) -> Option<I>;
}

#[async_trait::async_trait]
pub trait ActorO<O> {
    async fn send(&mut self, output: O) -> Result<(), ()>;
}

#[async_trait::async_trait]
pub trait ActorIO<I, O, RX: ActorI<I>, TX: ActorO<O>>: ActorI<I> + ActorO<O> {
    fn split(self) -> (RX, TX);
}
