use crate::{rpc::model::IntoVal, runtime::AsyncWrite, Requester};
use rmpv::Value;

#[derive(Clone)]
pub struct Buffer<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  pub(crate) code_data: Value,
  pub requester: Requester<W>,
}

impl<W> IntoVal<Value> for &Buffer<W>
where
  W: AsyncWrite + Send + Sync + Unpin + 'static,
{
  fn into_val(self) -> Value {
    self.code_data.clone()
  }
}