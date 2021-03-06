// Rust Bitcoin Library
// Written in 2014 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

#![macro_escape]

#[macro_export]
macro_rules! impl_serializable(
  ($thing:ident, $($field:ident),+) => (
    impl Serializable for $thing {
      fn serialize(&self) -> Vec<u8> {
        let mut ret = vec![];
        $( ret.extend(self.$field.serialize().move_iter()); )+
        ret
      }

      fn serialize_iter<'a>(&'a self) -> SerializeIter<'a> {
        SerializeIter {
          data_iter: None,
          sub_iter_iter: box vec![ $( &self.$field as &Serializable, )+ ].move_iter(),
          sub_iter: None,
          sub_started: false
        }
      }

      fn deserialize<I: Iterator<u8>>(mut iter: I) -> IoResult<$thing> {
        use util::misc::prepend_err;
        Ok($thing {
          $( $field: try!(prepend_err(stringify!($field), Serializable::deserialize(iter.by_ref()))), )+
        })
      }
    }
  );
)

#[macro_export]
macro_rules! impl_serializable_newtype(
  ($thing:ident, $parent:ty) => (
    impl Serializable for $thing {
      fn serialize(&self) -> Vec<u8> {
        let &$thing(ref data) = self;
        data.serialize()
      }

      fn deserialize<I: Iterator<u8>>(iter: I) -> IoResult<$thing> {
        let raw = Serializable::deserialize(iter);
        raw.map(|ok| $thing(ok))
      }
    }
  );
)

#[macro_export]
macro_rules! impl_message(
  ($thing:ident, $name:expr) => (
    impl $thing {
      /// Returns a human-readable description of the message
      fn command() -> String { String::from_str($name) }
    }

    impl Message for $thing {
      fn command(&self) -> String {
        $thing::command()
      }
    }
  );
)

