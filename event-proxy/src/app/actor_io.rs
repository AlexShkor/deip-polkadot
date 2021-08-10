
use tokio::sync::mpsc;

use crate::actor::*;


pub struct ActorJackI<I>(mpsc::Receiver<I>);
pub struct ActorJackO<O>(mpsc::Sender<O>);

pub struct ActorJack<I, O> {
    input: ActorJackI<I>,
    output: ActorJackO<O>,
}

impl<I, O> ActorJack<I, O> {
    pub fn pair() -> (Self, ActorJack<O, I>)
    {
        let (tx1, rx2) = mpsc::channel(1);
        let (tx2, rx1) = mpsc::channel(1);
        (Self { input: ActorJackI(rx1), output: ActorJackO(tx1) },
         ActorJack::<O, I> { input: ActorJackI(rx2), output: ActorJackO(tx2) })
    }
}

#[async_trait::async_trait]
impl<I: Send> ActorI<I> for ActorJackI<I> {
    async fn recv(&mut self) -> Option<I> {
        self.0.recv().await
    }
}

#[async_trait::async_trait]
impl<O: Send> ActorO<O> for ActorJackO<O> {
    async fn send(&mut self, output: O) -> Result<(), ()> {
        self.0.send(output).await.map_err(|_| ())
    }
}

#[async_trait::async_trait]
impl<I: Send, O: Send> ActorI<I> for ActorJack<I, O> {
    async fn recv(&mut self) -> Option<I> {
        self.input.recv().await
    }
}

#[async_trait::async_trait]
impl<I: Send, O: Send> ActorO<O> for ActorJack<I, O> {
    async fn send(&mut self, output: O) -> Result<(), ()> {
        self.output.send(output).await
    }
}

#[async_trait::async_trait]
impl<I: Send, O: Send> ActorIO<I, O, ActorJackI<I>, ActorJackO<O>> for ActorJack<I, O> {
    fn split(self) -> (ActorJackI<I>, ActorJackO<O>) {
        let Self { input, output } = self;
        (input, output)
    }
}
