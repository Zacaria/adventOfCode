use std::fmt;
use termion::{color, style};

#[derive(Clone, PartialEq)]
pub enum Cell<T> {
  Clean(T),
  Marked(T),
}

// impl<T> PartialEq for Cell<T> {
//   fn eq(&self, other: &Self) -> bool {
//     match self {
//       Cell
//     }
//   }
// }

impl <T: fmt::Display> fmt::Display for Cell<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Cell::Clean(v) => write!(f, "{}{: >2}{}", color::Fg(color::Blue), &v, style::Reset),
      Cell::Marked(v) => write!(f, "{}{}{}", color::Fg(color::Red), &v, style::Reset),
    }
  }
}

impl <T: fmt::Display> fmt::Debug for Cell<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Cell::Clean(v) => write!(f, "{}{: >2}{}", color::Fg(color::Blue), &v, style::Reset),
      Cell::Marked(v) => write!(f, "{}{}{}", color::Fg(color::Red), &v, style::Reset),
    }
  }
}