use std::{collections::VecDeque, task::Poll};

use futures::Stream;

use crate::number::{digit::Digit, int::BigInt};


pub trait IntoView {
    fn into_view(&self) -> View;
}
impl BigInt{
    pub async fn into_view(&self) -> View {
        let vec = self.data.read().await;
        vec.split_at(vec.len()/2).into_view()
    }
}
impl IntoView for (&[Digit], &[Digit]){
    fn into_view(&self) -> View {
        View {
            data: match self.to_owned(){
                (left, right) =>{
                    (left.to_vec().into(), right.to_vec().into())
                }
            }
        }
    }
}
pub struct View{
    data: (VecDeque<Digit>, Vec<Digit>)
}
impl Stream for View{
    type Item = (Option<Digit>, Option<Digit>);
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let data = &mut self.get_mut().data;
        match (data.0.pop_front(), data.1.pop()){
            (None, None) => Poll::Ready(None),
            (left, right) => Poll::Ready(Some((left, right))),
        }
    }
}