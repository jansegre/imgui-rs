use std::marker::PhantomData;

use super::ffi;
use super::{Frame, ImStr};

// TODO: Consider using Range, even though it is half-open

pub struct SliderInt<'fr, 'p> {
   label: ImStr<'p>,
   value: &'p mut i32,
   min: i32,
   max: i32,
   display_format: ImStr<'p>,
   _phantom: PhantomData<&'fr Frame<'fr>>
}

impl<'fr, 'p> SliderInt<'fr, 'p> {
   pub fn new(label: ImStr<'p>, value: &'p mut i32, min: i32, max: i32) -> Self {
      SliderInt {
         label: label,
         value: value,
         min: min,
         max: max,
         display_format: unsafe { ImStr::from_bytes(b"%.0f\0") },
         _phantom: PhantomData
      }
   }
   #[inline]
   pub fn display_format(self, display_format: ImStr<'p>) -> Self {
      SliderInt {
         display_format: display_format,
         .. self
      }
   }
   pub fn build(self) -> bool {
      unsafe {
         ffi::igSliderInt(self.label.as_ptr(), self.value, self.min, self.max,
            self.display_format.as_ptr()
         )
      }
   }
}

pub struct SliderFloat<'fr, 'p> {
   label: ImStr<'p>,
   value: &'p mut f32,
   min: f32,
   max: f32,
   display_format: ImStr<'p>,
   power: f32,
   _phantom: PhantomData<&'fr Frame<'fr>>
}

impl<'fr, 'p> SliderFloat<'fr, 'p> {
   pub fn new(label: ImStr<'p>, value: &'p mut f32, min: f32, max: f32) -> Self {
      SliderFloat {
         label: label,
         value: value,
         min: min,
         max: max,
         display_format: unsafe { ImStr::from_bytes(b"%.3f\0") },
         power: 1.0,
         _phantom: PhantomData
      }
   }
   #[inline]
   pub fn display_format(self, display_format: ImStr<'p>) -> Self {
      SliderFloat {
         display_format: display_format,
         .. self
      }
   }
   #[inline]
   pub fn power(self, power: f32) -> Self {
      SliderFloat {
         power: power,
         .. self
      }
   }
   pub fn build(self) -> bool {
      unsafe {
         ffi::igSliderFloat(self.label.as_ptr(), self.value, self.min, self.max,
            self.display_format.as_ptr(),
            self.power
         )
      }
   }
}

