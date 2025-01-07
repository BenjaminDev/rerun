// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/view_coordinates.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: How we interpret the coordinate system of an entity/space.
///
/// For instance: What is "up"? What does the Z axis mean?
///
/// The three coordinates are always ordered as [x, y, z].
///
/// For example [Right, Down, Forward] means that the X axis points to the right, the Y axis points
/// down, and the Z axis points forward.
///
/// ⚠ [Rerun does not yet support left-handed coordinate systems](https://github.com/rerun-io/rerun/issues/5032).
///
/// The following constants are used to represent the different directions:
///  * Up = 1
///  * Down = 2
///  * Right = 3
///  * Left = 4
///  * Forward = 5
///  * Back = 6
#[derive(Clone, Debug, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct ViewCoordinates(
    /// The directions of the [x, y, z] axes.
    pub crate::datatypes::ViewCoordinates,
);

impl ::re_types_core::Component for ViewCoordinates {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("rerun.components.ViewCoordinates")
    }
}

::re_types_core::macros::impl_into_cow!(ViewCoordinates);

impl ::re_types_core::Loggable for ViewCoordinates {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::datatypes::ViewCoordinates::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::ViewCoordinates::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::ViewCoordinates::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::ViewCoordinates::from_arrow(arrow_data).map(bytemuck::cast_vec)
    }
}

impl<T: Into<crate::datatypes::ViewCoordinates>> From<T> for ViewCoordinates {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::ViewCoordinates> for ViewCoordinates {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::ViewCoordinates {
        &self.0
    }
}

impl std::ops::Deref for ViewCoordinates {
    type Target = crate::datatypes::ViewCoordinates;

    #[inline]
    fn deref(&self) -> &crate::datatypes::ViewCoordinates {
        &self.0
    }
}

impl std::ops::DerefMut for ViewCoordinates {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::ViewCoordinates {
        &mut self.0
    }
}

impl ::re_byte_size::SizeBytes for ViewCoordinates {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::ViewCoordinates>::is_pod()
    }
}
