// Copyright 2020 The druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Support for lenses, a way of focusing on subfields of data.
//!
//! Lenses are useful whenever a widget only needs access to a subfield of a larger struct or
//! generally access to part of a larger value.
//!
//! For example: If one wants to embed a [`TextBox`] in a widget with a `Data` type
//! that is not `String`, they need to specify how to access a `String` from within the `Data`.
//!
//! [`TextBox`]: ../widget/struct.TextBox.html
//! ```
//! use druid::{Data, Lens, Widget, WidgetExt, widget::{TextBox, Flex}};
//!
//! #[derive(Clone, Debug, Data, Lens)]
//! struct MyState {
//!     search_term: String,
//!     scale: f64,
//!     // ...
//! }
//!
//!
//! fn my_sidebar() -> impl Widget<MyState> {
//!     // `TextBox` is of type `Widget<String>`
//!     // via `.lens` we get it to be of type `Widget<MyState>`.
//!     // `MyState::search_term` is a lens generated by the `derive(Lens)` macro,
//!     // that provides access to the search_term field.
//!     let searchbar = TextBox::new().lens(MyState::search_term);
//!
//!     // ...
//!
//!     // We can now use `searchbar` just like any other `Widget<MyState>`
//!     Flex::column().with_child(searchbar)
//! }
//! ```

#[allow(clippy::module_inception)]
mod lens;
pub use lens::{Deref, Field, Id, InArc, Index, Map, Then};
#[doc(hidden)]
pub use lens::{Lens, LensExt, LensWrap};
