#[async_trait::async_trait]
pub trait ActorI<I> {
    async fn recv(&mut self) -> Option<I>;
}

#[async_trait::async_trait]
pub trait ActorO<O> {
    async fn send(&mut self, output: O) -> Result<(), ()>;
}

#[async_trait::async_trait]
pub trait ActorIO<I, O, RX, TX, RX2, TX2>: ActorI<I> + ActorO<O> + Sized
    where
        RX: ActorI<I>, TX: ActorO<O>, RX2: ActorI<O>, TX2: ActorO<I>
{
    type Pair: ActorIO<O, I, RX2, TX2, RX, TX>;
    
    fn pair() -> (Self, Self::Pair);
    
    fn split(self) -> (RX, TX);
}
