#![cfg_attr(docsrs, feature(doc_cfg))]

mod std_io_traits;
pub use std_io_traits::{Save, Load};
// #[cfg(feature = "async-tokio")]
// mod tokio_io_traits;
// #[cfg(feature = "async-tokio")]
// #[cfg_attr(docsrs, doc(cfg(feature = "async-tokio")))]
// pub use tokio_io_traits::{AsyncSave, AsyncLoad};
