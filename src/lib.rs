// TODO: cap을 무조건 u64로 저장한다는걸 README에 명시해야함. 그 이유까지. u32로 저장하는걸 feature로 두자.

mod std_io_traits;
pub use std_io_traits::{Save, Load};
#[cfg(feature = "async-tokio")]
mod tokio_io_traits;
#[cfg(feature = "async-tokio")]
pub use tokio_io_traits::{AsyncSave, AsyncLoad};
