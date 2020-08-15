/// ## section_data
/// A submodule containing the section data container of the doctree.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### SectionData
/// A container for section-related data of the doctree of ruSTLa.
pub struct SectionData {

  /// #### section_levels
  /// A mapping of the different encountered section styles to section levels.
  section_levels: HashMap<SectionLineStyle, usize>,

  /// #### highest_encountered_section_level
  /// As the name implies, this counter is incremented as new types of sections
  /// are encountered in the document. It is assigned to `self.levels` when a new
  /// type is encountered and incremented.
  highest_encountered_section_level: usize,
}


impl SectionData {

  /// ### new
  /// A `SectionData` constructor.
  pub fn new () -> Self{
    Self {
      section_levels: HashMap::new(),
      highest_encountered_section_level: 0
    }
  }

  /// ### increment_encountered_section_number
  /// Increments the number of encoutnered sections.
  pub fn increment_encountered_section_number (&mut self) {
    self.highest_encountered_section_level += 1;
  }


  /// ### line_style_section_level
  /// Asks the SectionData container for the section level that a given section
  /// line style corresponds to. If the line style has not been encountered before,
  /// returns `self.highest_encountered_section_level + 1`.
  pub fn line_style_section_level (&self, line_style: &SectionLineStyle) -> usize {
    match self.section_levels.get(line_style) {
      Some(section_level) => *section_level,
      None => self.highest_encountered_section_level
    }
  }


  /// ### add_section_level
  /// Adds a new section line style to known section levels.
  /// Simply panics, if a section style is updated (for now).
  pub fn add_section_level (&mut self, section_style: SectionLineStyle) {

    match self.section_levels.insert(section_style, self.highest_encountered_section_level + 1) {
      Some(level) => panic!("Tried updating level of section style {:#?}", section_style),
      None => {}
    }
    self.increment_encountered_section_number()
  }

}