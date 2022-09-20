use std::{borrow::Cow, ffi, os::raw::c_char};

pub fn char_slice_to_cow(chars: &[c_char]) -> Cow<'_, str> {
  unsafe { String::from_utf8_lossy(ffi::CStr::from_ptr(chars.as_ptr()).to_bytes()) }
}

pub unsafe fn chars_to_cow<'a>(chars: *const c_char) -> Cow<'a, str> {
  String::from_utf8_lossy(ffi::CStr::from_ptr(chars).to_bytes())
}

#[inline]
pub fn cow_to_string(cow: Cow<str>) -> String {
  cow.into_owned()
}

macro_rules! to_c_string {
  ($v:expr) => {
    ffi::CString::new($v)?.as_ptr().cast::<std::os::raw::c_char>()
  };
}

macro_rules! as_ref {
  ($from:ident -> $to:ty, $self:ident . $field:ident) => {
    as_ref!(@ $from -> $to, , $self, $self.$field);
  };

  ($from:ident -> $to:ty, * $self:ident . $field:ident) => {
    as_ref!(@ $from -> $to, unsafe, $self, *$self.$field);
  };

  (@ $from:ident -> $to:ty, $($unsafe:ident)?, $self:ident, $value:expr) => {
    impl AsRef<$to> for $from {
      fn as_ref(&$self) -> &$to {
        $($unsafe)? { & $value }
      }
    }

    impl AsMut<$to> for $from {
      fn as_mut(&mut $self) -> &mut $to {
        $($unsafe)? { &mut $value }
      }
    }
  };
}

macro_rules! bitflags {
  ($(# $attr:tt)* $name:ident = $target:ident { $($(# $field_attr:tt)* $field:ident: $value:ident,)* }) => {
    $(# $attr)*
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct $name(libgphoto2_sys::$target);

    impl From<libgphoto2_sys::$target> for $name {
      fn from(flags: libgphoto2_sys::$target) -> Self {
        Self(flags)
      }
    }

    impl $name {
      $(
        $(# $field_attr)*
        #[inline]
        pub fn $field(&self) -> bool {
          (self.0 & libgphoto2_sys::$target::$value).0 != 0
        }
      )*
    }

    impl std::fmt::Debug for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!($name))
          $(
            .field(stringify!($field), &self.$field())
          )*
          .finish()
      }
    }
  };
}

pub(crate) use {as_ref, bitflags, to_c_string};
