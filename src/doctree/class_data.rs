/// ## class_data
///
/// A submodule that defines a HTML class container `ClassData`, that a `DocTree` holds on to.
///
/// Copyright (c) 2020, Santtu Söderholm <santtu.soderholm@tuni.fi>


/// ### ClassData
///
/// A container for HTML classes encountered in the reStructuredText document being parsed.
pub struct ClassData {
  incoming_classes: Vec<String>
}


impl ClassData {

  pub fn new () -> Self {
    Self {
      incoming_classes: Vec::new()
    }
  }

  /// ### push_class
  ///
  /// Creates a string out of a given string slice and adds it to incoming classes,
  /// that will be assignemd to the next non-comment node.
  pub fn push_class (&mut self, class: &str) {
    self.incoming_classes.push(class.to_string())
  }

  /// Retrieves a shared reference to the contained incoming class vector.
  pub fn shared_classes (&self) -> &Vec<String> {
    &self.incoming_classes
  }

  /// Retrieves a mutable reference to the contained incoming class vector.
  pub fn mut_classes (&mut self) -> &mut Vec<String> {
    &mut self.incoming_classes
  }

}