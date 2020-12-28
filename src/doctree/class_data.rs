/*!
A submodule that defines a HTML class container `ClassData`, that a `DocTree` holds on to.
If a `class` reST directive is encountered, the classes specified by it
are stored in the `ClassData` container. If a reST text element *other* than `class`
is ran into, the classes stored in the container are given to the node corresponding
to thetext element.

(c) 2020, Santtu Söderholm <santtu.soderholm@tuni.fi>
*/

/// A container for HTML classes encountered in the reStructuredText document being parsed.
pub struct ClassData {
    incoming_classes: Vec<String>,
}

impl ClassData {
    pub fn new() -> Self {
        Self {
            incoming_classes: Vec::new(),
        }
    }

    /// ### push_class
    ///
    /// Creates a string out of a given string slice and adds it to incoming classes,
    /// that will be assignemd to the next non-comment node.
    pub fn push_class(&mut self, class: &str) {
        self.incoming_classes.push(class.to_string())
    }

    /// Retrieves a shared reference to the contained incoming class vector.
    pub fn shared_classes(&self) -> &Vec<String> {
        &self.incoming_classes
    }

    /// Retrieves a mutable reference to the contained incoming class vector.
    pub fn mut_classes(&mut self) -> &mut Vec<String> {
        &mut self.incoming_classes
    }
}
