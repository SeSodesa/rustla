/// ## utf8_to_latex
/// 
/// This file contains a mapping between a subset of UTF-8 to LaTeX commands.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi


use std::collections::HashMap;
use lazy_static::lazy_static;


/// str_to_latex
/// 
/// A function for converting a given `&str` (which is valid UTF-8)
/// into a valid LaTeX string. Some more exotic symbols might require
/// a specific LaTeX package fo the resulting object code to parse without errors,
/// which is *not* taken into account by this function.
/// 
/// If not conversion exists, adds the unicode scalar into the string unchanged.
pub fn unicode_math_to_latex (utf_str: &str) -> String {

  let source_char_count = utf_str.chars().count();
  let mut latex_string = String::with_capacity(source_char_count);

  for c in utf_str.chars() {
    if let Some(latex_str) = UTF8_MATH_TO_LATEX_MAP.get(&c) {
      latex_string += latex_str;
    } else {
      latex_string.push(c);
    }
  }

  latex_string
}


/// ### unicode_text_to_latex
///
/// Escapes any non-text category LaTeX characters in a given `&str`,
/// so as to allow the generated `.tex` document to compile if it has
/// control characters such as underscores in a text node.
/// 
/// For example, `'_' ↦ "\_"` and `'@' ↦ "\@"`. If a character is not recognized as a control character,
/// it is added to the generated `String` as is.
pub fn unicode_text_to_latex (utf_str: &str) -> String {

  let source_char_count = utf_str.chars().count();
  let mut latex_string = String::with_capacity(source_char_count);

  for c in utf_str.chars() {
    if let Some(latex_str) = UTF8_TEXT_TO_LATEX_MAP.get(&c) {
      latex_string += latex_str;
    } else {
      latex_string.push(c);
    }
  }

  latex_string
}


lazy_static! {

  /// ### UTF8_MATH_TO_LATEX_MAP
  /// 
  /// A mapping of certain utf8 scalars to LaTeX strings.
  /// 
  /// source: http://milde.users.sourceforge.net/LUCR/Math/unimathsymbols.html, 2020-09-15
  static ref UTF8_MATH_TO_LATEX_MAP: HashMap<char, &'static str> = {
    let mut map = HashMap::new();

    // Basic Latin
    map.insert('\u{23}', r#"\#"#);
    map.insert('\u{24}', r#"\$"#);
    map.insert('\u{25}', r#"\%"#);
    map.insert('\u{26}', r#"\&"#);
    // map.insert('\u{5c}', r#"\backslash"#);
    // map.insert('\u{7b}', r#"\{"#);
    // map.insert('\u{7d}', r#"\}"#);
    map.insert('\u{7e}', r#"\sim"#);

    // Latin 1 Supplement
    map.insert('\u{a2}', r#"\cent"#);
    map.insert('\u{a3}', r#"\pounds"#);
    map.insert('\u{a5}', r#"\yen"#);
    map.insert('\u{a8}', r#"\spddot"#);
    map.insert('\u{ac}', r#"\neg"#);
    map.insert('\u{b1}', r#"\pm"#);
    map.insert('\u{b5}', r#"\mathrm{\mu}"#);
    map.insert('\u{b7}', r#"\cdot"#);
    map.insert('\u{d7}', r#"\times"#);
    map.insert('\u{f7}', r#"\eth"#);

    // Combining diacritics
    map.insert('\u{300}', r#"\grave"#);
    map.insert('\u{301}', r#"\acute"#);
    map.insert('\u{302}', r#"\hat"#);
    map.insert('\u{303}', r#"\tilde"#);
    map.insert('\u{304}', r#"\bar"#);
    map.insert('\u{305}', r#"\overline"#);
    map.insert('\u{306}', r#"\breve"#);
    map.insert('\u{307}', r#"\dot"#);
    map.insert('\u{308}', r#"\ddot"#);
    map.insert('\u{30a}', r#"\mathring"#);
    map.insert('\u{30c}', r#"\check"#);
    //
    map.insert('\u{330}', r#"\utilde"#);
    map.insert('\u{331}', r#"\underbar"#);
    map.insert('\u{332}', r#"\underline"#);
    map.insert('\u{338}', r#"\not"#);


    // Greek and Coptic
    map.insert('\u{391}', r#"A"#);
    map.insert('\u{392}', r#"B"#);
    map.insert('\u{393}', r#"\Gamma"#);
    map.insert('\u{394}', r#"\Delta"#);
    map.insert('\u{395}', r#"E"#);
    map.insert('\u{396}', r#"Z"#);
    map.insert('\u{397}', r#"H"#);
    map.insert('\u{398}', r#"\Theta"#);
    map.insert('\u{399}', r#"I"#);
    map.insert('\u{39a}', r#"K"#);
    map.insert('\u{39b}', r#"\Lambda"#);
    map.insert('\u{39c}', r#"M"#);
    map.insert('\u{39d}', r#"N"#);
    map.insert('\u{39e}', r#"\Xi"#);
    map.insert('\u{39f}', r#"O"#);
    map.insert('\u{3a0}', r#"\Pi"#);
    map.insert('\u{3a1}', r#"P"#);
    map.insert('\u{3a3}', r#"\Sigma"#);
    map.insert('\u{3a4}', r#"T"#);
    map.insert('\u{3a5}', r#"\Upsilon"#);
    map.insert('\u{3a6}', r#"\Phi"#);
    map.insert('\u{3a7}', r#"X"#);
    map.insert('\u{3a8}', r#"\Psi"#);
    map.insert('\u{3a8}', r#"\Omega"#);
    map.insert('\u{3b1}', r#"\alpha"#);
    map.insert('\u{3b2}', r#"\beta"#);
    map.insert('\u{3b3}', r#"\gamma"#);
    map.insert('\u{3b4}', r#"\delta"#);
    map.insert('\u{3b5}', r#"\varepsilon"#);
    map.insert('\u{3b6}', r#"\zeta"#);
    map.insert('\u{3b7}', r#"\eta"#);
    map.insert('\u{3b8}', r#"\theta"#);
    map.insert('\u{3b9}', r#"\iota"#);
    map.insert('\u{3ba}', r#"\kappa"#);
    map.insert('\u{3bb}', r#"\lambda"#);
    map.insert('\u{3bc}', r#"\mu"#);
    map.insert('\u{3bd}', r#"\nu"#);
    map.insert('\u{3be}', r#"\xi"#);
    map.insert('\u{3bf}', r#"o"#);
    map.insert('\u{3c0}', r#"\pi"#);
    map.insert('\u{3c1}', r#"\rho"#);
    map.insert('\u{3c2}', r#"\varsigma"#);
    map.insert('\u{3c3}', r#"\sigma"#);
    map.insert('\u{3c4}', r#"\tau"#);
    map.insert('\u{3c5}', r#"\upsilon"#);
    map.insert('\u{3c6}', r#"\varphi"#);
    map.insert('\u{3c7}', r#"\chi"#);
    map.insert('\u{3c8}', r#"\psi"#);
    map.insert('\u{3c9}', r#"\omega"#);
    map.insert('\u{3d0}', r#"\varbeta"#);
    map.insert('\u{3d1}', r#"\vartheta"#);
    map.insert('\u{3d2}', r#"\Upsilon"#); // actually \mathrm{\Upsilon}
    map.insert('\u{3d5}', r#"\phi"#);
    map.insert('\u{3d6}', r#"\varpi"#);
    map.insert('\u{3d8}', r#"\Qoppa"#);
    map.insert('\u{3d9}', r#"\qoppa"#);
    map.insert('\u{3da}', r#"\Stigma"#);
    map.insert('\u{3db}', r#"\stigma"#);
    map.insert('\u{3dc}', r#"\Digamma"#);
    map.insert('\u{3dd}', r#"\digamma"#);
    map.insert('\u{3de}', r#"\Koppa"#);
    map.insert('\u{3df}', r#"\koppa"#);
    map.insert('\u{3e0}', r#"\Sampi"#);
    map.insert('\u{3e1}', r#"\sampi"#);
    map.insert('\u{3f0}', r#"\varkappa"#);
    map.insert('\u{3f1}', r#"\varrho"#);
    map.insert('\u{3f4}', r#"\Theta"#); // actually \Vartheta
    map.insert('\u{3f5}', r#"\epsilon"#);
    map.insert('\u{3f6}', r#"\backepsilon"#);

    // General punctuation
    map.insert('\u{2000}', r#"\ "#);
    map.insert('\u{2001}', r#"\quad"#);
    map.insert('\u{2002}', r#"\qquad"#);
    map.insert('\u{2003}', r#"\ "#);
    map.insert('\u{2004}', r#"\ "#);
    map.insert('\u{2005}', r#"\ "#);
    map.insert('\u{2006}', r#"\ "#);
    map.insert('\u{2007}', r#"\ "#);
    map.insert('\u{2008}', r#"\ "#);
    map.insert('\u{2009}', r#"\ "#);
    map.insert('\u{200a}', r#"\ "#);
    map.insert('\u{200a}', r#"\ "#);
    map.insert('\u{2010}', r#"-"#);
    map.insert('\u{2012}', r#"-"#);
    map.insert('\u{2013}', r#"-"#);
    map.insert('\u{2014}', r#"-"#);
    map.insert('\u{2015}', r#"-"#);
    map.insert('\u{2016}', r#"\Vert"#);
    //
    map.insert('\u{2020}', r#"\dagger"#);
    map.insert('\u{2021}', r#"\\ddagger"#);
    map.insert('\u{2022}', r#"\bullet"#);
    //
    map.insert('\u{2026}', r#"\ldots"#);
    map.insert('\u{2032}', r#"\prime"#);
    map.insert('\u{2033}', r#"\second"#);
    map.insert('\u{2034}', r#"\third"#);
    map.insert('\u{2035}', r#"\backprime"#);
    //
    map.insert('\u{203c}', r#"!!"#);
    map.insert('\u{2040}', r#"\cat"#);
    //
    map.insert('\u{2044}', r#"/"#);
    map.insert('\u{2047}', r#"??"#);
    //
    map.insert('\u{2052}', r#"\:"#);
    

    // Super- and subscripts
    map.insert('\u{207a}', r#"^{+}"#);
    map.insert('\u{207b}', r#"^{-}"#);
    map.insert('\u{207c}', r#"^{=}"#);
    map.insert('\u{207d}', r#"^{(}"#);
    map.insert('\u{207e}', r#"^{)}"#);
    map.insert('\u{208a}', r#"^{+}"#);
    map.insert('\u{208b}', r#"^{-}"#);
    map.insert('\u{208c}', r#"^{=}"#);
    map.insert('\u{208d}', r#"^{(}"#);
    map.insert('\u{208e}', r#"^{)}"#);

    // Letterlike symbols
    map.insert('\u{2102}', r#"\mathbb{C}"#);
    map.insert('\u{2107}', r#"\Euler"#);
    map.insert('\u{210a}', r#"\mathcal{g}"#);
    map.insert('\u{210b}', r#"\mathcal{H}"#);
    map.insert('\u{210c}', r#"\mathfrak{H}"#);
    map.insert('\u{210d}', r#"\mathbb{H}"#);
    map.insert('\u{210e}', r#"h"#);
    map.insert('\u{210f}', r#"\hslash"#);
    map.insert('\u{2110}', r#"\mathcal{I}"#);
    map.insert('\u{2111}', r#"\Im"#);
    map.insert('\u{2112}', r#"\mathcal{L}"#);
    map.insert('\u{2113}', r#"\ell"#);
    map.insert('\u{2115}', r#"\mathbb{N}"#);
    map.insert('\u{2118}', r#"\wp"#);
    map.insert('\u{2119}', r#"\mathbb{P}"#);
    map.insert('\u{211a}', r#"\mathbb{Q}"#);
    map.insert('\u{211b}', r#"\mathcal{R}"#);
    map.insert('\u{211c}', r#"\Re"#);
    map.insert('\u{211d}', r#"\mathbb{R}"#);
    map.insert('\u{2124}', r#"\mathbb{Z}"#);
    map.insert('\u{2126}', r#"\Omega"#);
    map.insert('\u{2127}', r#"\mho"#);
    map.insert('\u{2128}', r#"\mathfrak{Z}"#);
    //
    map.insert('\u{212b}', r#"\Angstroem"#);
    map.insert('\u{212c}', r#"\mathcal{B}"#);
    map.insert('\u{212d}', r#"\mathfrak{C}"#);
    map.insert('\u{212f}', r#"\mathcal{e}"#);
    map.insert('\u{2130}', r#"\mathcal{E}"#);
    map.insert('\u{2131}', r#"\mathcal{F}"#);
    map.insert('\u{2132}', r#"\Finv"#);
    map.insert('\u{2133}', r#"\mathcal{M}"#);
    map.insert('\u{2134}', r#"\mathcal{o}"#);
    map.insert('\u{2135}', r#"\aleph"#);
    map.insert('\u{2136}', r#"\beth"#);
    map.insert('\u{2137}', r#"\gimel"#);
    map.insert('\u{2138}', r#"\daleth"#);
    map.insert('\u{213c}', r#"\mathbb{pi}"#);
    map.insert('\u{213d}', r#"\mathbb{gamma}"#);
    map.insert('\u{213e}', r#"\mathbb{Gamma}"#);
    map.insert('\u{213f}', r#"\mathbb{Pi}"#);
    map.insert('\u{2140}', r#"\mathbb{Sigma}"#);
    //
    map.insert('\u{2144}', r#"\Yup"#);
    map.insert('\u{2145}', r#"\CapitalDifferentialD"#);
    map.insert('\u{2146}', r#"\DifferentialD"#);
    map.insert('\u{2147}', r#"\ExponentialE"#);
    map.insert('\u{2148}', r#"\ComplexI"#);
    map.insert('\u{2149}', r#"\ComplexJ"#);
    //
    map.insert('\u{214b}', r#"\invamp"#);


    // Arrows
    map.insert('\u{2190}', r#"\leftarrow"#);
    map.insert('\u{2191}', r#"\uparrow"#);
    map.insert('\u{2192}', r#"\rightarrow"#);
    map.insert('\u{2193}', r#"\downarrow"#);
    map.insert('\u{2194}', r#"\leftrightarrow"#);
    map.insert('\u{2195}', r#"\nwwnarrow"#);
    map.insert('\u{2196}', r#"\nearrow"#);
    map.insert('\u{2197}', r#"\searrow"#);
    map.insert('\u{2198}', r#"\swarrow"#);
    map.insert('\u{2199}', r#"\nleftarrow"#);
    map.insert('\u{219a}', r#"\nrightarrow"#);
    //
    map.insert('\u{219e}', r#"\twoheadleftarrow"#);
    //
    map.insert('\u{21a0}', r#"\twoheadrightarrow"#);
    //
    map.insert('\u{21a2}', r#"\leftarrowtail"#);
    map.insert('\u{21a3}', r#"\rightarrowtail"#);
    map.insert('\u{21a4}', r#"\mapsfrom"#);
    map.insert('\u{21a5}', r#"\MapsUp"#);
    map.insert('\u{21a6}', r#"\mapsto"#);
    map.insert('\u{21a7}', r#"\MapsDown"#);
    //
    map.insert('\u{21a9}', r#"\heekleftarrow"#);
    map.insert('\u{21aa}', r#"\hookrightarrow"#);
    map.insert('\u{21ab}', r#"\looparrowleft"#);
    map.insert('\u{21ac}', r#"\looparrowright"#);
    map.insert('\u{21ad}', r#"\leftrightsquigarrow"#);
    map.insert('\u{21ae}', r#"\nleftrightarrow"#);
    map.insert('\u{21af}', r#"\lightning"#);
    //
    map.insert('\u{21b6}', r#"\curvearrowleft"#);
    map.insert('\u{21b7}', r#"\curvearrowright"#);
    //
    map.insert('\u{21ba}', r#"\circlearrowleft"#);
    map.insert('\u{21bb}', r#"\circlearrowright"#);
    map.insert('\u{21bc}', r#"\leftharpoonup"#);
    map.insert('\u{21bd}', r#"\leftharpoondown"#);
    map.insert('\u{21be}', r#"\upharpoonright"#);
    map.insert('\u{21bf}', r#"\upharpoonleft"#);
    map.insert('\u{21c0}', r#"\rightharpoonup"#);
    map.insert('\u{21c1}', r#"\rightharpoondown"#);
    map.insert('\u{21c2}', r#"\downharpoonright"#);
    map.insert('\u{21c3}', r#"\downharpoonleft"#);
    map.insert('\u{21c4}', r#"\rightleftarrows"#);
    map.insert('\u{21c5}', r#"\updownarrows"#);
    map.insert('\u{21c6}', r#"\leftrightarrows"#);
    map.insert('\u{21c7}', r#"\leftleftarrows"#);
    map.insert('\u{21c8}', r#"\upuparrows"#);
    map.insert('\u{21c9}', r#"\rightrightarrows"#);
    map.insert('\u{21ca}', r#"\downdownarrows"#);
    map.insert('\u{21cb}', r#"\leftrightharpoons"#);
    map.insert('\u{21cc}', r#"\rightleftharpoons"#);
    map.insert('\u{21cd}', r#"\nLeftarrow"#);
    map.insert('\u{21ce}', r#"\nLeftrightarrow"#);
    map.insert('\u{21cf}', r#"\nRightarrow"#);
    map.insert('\u{21cf}', r#"\nRightarrow"#);
    map.insert('\u{21d0}', r#"\Leftarrow"#);
    map.insert('\u{21d1}', r#"\Uparrow"#);
    map.insert('\u{21d2}', r#"\Rightarrow"#);
    map.insert('\u{21d3}', r#"\Downarrow"#);
    map.insert('\u{21d4}', r#"\Leftrightarrow"#);
    map.insert('\u{21d5}', r#"\Updownarrowarrow"#);
    map.insert('\u{21d6}', r#"\Nwarrow"#);
    map.insert('\u{21d7}', r#"\Searrow"#);
    map.insert('\u{21d8}', r#"\Swarrow"#);
    map.insert('\u{21da}', r#"\Lleftarrow"#);
    map.insert('\u{21db}', r#"\Rrightarrow"#);
    map.insert('\u{21dc}', r#"\leftsquigarrow"#);
    map.insert('\u{21dd}', r#"\rightsquigarrow"#);
    //
    map.insert('\u{21f5}', r#"\downuparrows"#);


    // Mathematical operators
    map.insert('\u{2200}', r#"\forall"#);
    map.insert('\u{2201}', r#"\complement"#);
    map.insert('\u{2202}', r#"\partial"#);
    map.insert('\u{2203}', r#"\exists"#);
    map.insert('\u{2204}', r#"\nexists"#);
    map.insert('\u{2205}', r#"\varnothing"#);
    map.insert('\u{2206}', r#"\Delta"#); // \mathrm{\Delta}
    map.insert('\u{2207}', r#"\nabla"#);
    map.insert('\u{2208}', r#"\in"#);
    map.insert('\u{2209}', r#"\notin"#);
    map.insert('\u{220a}', r#"\epsilon"#);
    map.insert('\u{220b}', r#"\ni"#);
    map.insert('\u{220c}', r#"\nni"#);
    //
    map.insert('\u{220f}', r#"\prod"#);
    map.insert('\u{2210}', r#"\coprod"#);
    map.insert('\u{2211}', r#"\sum"#);
    map.insert('\u{2212}', r#"-"#);
    map.insert('\u{2213}', r#"\mp"#);
    map.insert('\u{2214}', r#"\dotplus"#);
    map.insert('\u{2215}', r#"\slash"#);
    map.insert('\u{2216}', r#"\smallsetminus"#);
    map.insert('\u{2217}', r#"\ast"#);
    map.insert('\u{2218}', r#"\circ"#);
    map.insert('\u{2219}', r#"\bullet"#);
    map.insert('\u{221a}', r#"\sqrt"#);
    map.insert('\u{221b}', r#"\sqrt[3]"#);
    map.insert('\u{221c}', r#"\sqrt[4]"#);
    map.insert('\u{221d}', r#"\propto"#);
    map.insert('\u{221e}', r#"\infty"#);
    //
    map.insert('\u{2227}', r#"\wedge"#);
    map.insert('\u{2228}', r#"\vee"#);
    map.insert('\u{2229}', r#"\cap"#);
    map.insert('\u{222a}', r#"\cup"#);
    map.insert('\u{222b}', r#"\int"#);
    map.insert('\u{222c}', r#"\iint"#);
    map.insert('\u{222d}', r#"\iiint"#);
    map.insert('\u{222e}', r#"\oint"#);
    map.insert('\u{222f}', r#"\oiint"#);
    map.insert('\u{2230}', r#"\oiiint"#);
    //
    map.insert('\u{2234}', r#"\therefore"#);
    map.insert('\u{2235}', r#"\because"#);
    map.insert('\u{2236}', r#":"#);
    map.insert('\u{2237}', r#"\Proportion"#);
    //
    map.insert('\u{223c}', r#"\sim"#);
    map.insert('\u{223d}', r#"\backsim"#);
    //
    map.insert('\u{2241}', r#"\nsim"#);
    //
    map.insert('\u{2248}', r#"\approx"#);
    map.insert('\u{2249}', r#"\napprox"#);
    //
    map.insert('\u{2254}', r#"\coloneq"#);
    map.insert('\u{2255}', r#"\eqcolon"#);
    //
    map.insert('\u{2259}', r#"\corresponds"#);
    //
    map.insert('\u{2260}', r#"\neq"#);
    map.insert('\u{2261}', r#"\equiv"#);
    map.insert('\u{2262}', r#"\nequiv"#);
    //
    map.insert('\u{2264}', r#"\leq"#);
    map.insert('\u{2265}', r#"\geq"#);
    //
    map.insert('\u{2264}', r#"\ll"#);
    map.insert('\u{2264}', r#"\gg"#);
    //
    map.insert('\u{226e}', r#"\nless"#);
    map.insert('\u{226f}', r#"\ngtr"#);
    map.insert('\u{2270}', r#"\nleq"#);
    map.insert('\u{2271}', r#"\ngeq"#);
    //
    map.insert('\u{227a}', r#"\prec"#);
    map.insert('\u{227b}', r#"\succ"#);
    map.insert('\u{2270}', r#"\preccurlyeq"#);
    map.insert('\u{2270}', r#"\succurlyeq"#);
    map.insert('\u{2270}', r#"\precsim"#);
    map.insert('\u{2270}', r#"\succsim"#);
    map.insert('\u{2270}', r#"\nprec"#);
    map.insert('\u{2270}', r#"\nsucc"#);

    map.insert('\u{2282}', r#"\subset"#);
    map.insert('\u{2283}', r#"\supset"#);
    map.insert('\u{2284}', r#"\nsubset"#);
    map.insert('\u{2285}', r#"\nsupset"#);
    map.insert('\u{2286}', r#"\subseteq"#);
    map.insert('\u{2287}', r#"\supseteq"#);
    map.insert('\u{2288}', r#"\nsubseteq"#);
    map.insert('\u{2289}', r#"\nsupseteq"#);
    map.insert('\u{228a}', r#"\subsetneq"#);
    map.insert('\u{228b}', r#"\supsetneq"#);
    //
    map.insert('\u{22a2}', r#"\vdash"#);
    map.insert('\u{22a3}', r#"\dashv"#);
    map.insert('\u{22a4}', r#"\top"#);
    map.insert('\u{22a5}', r#"\bot"#);
    //
    map.insert('\u{22a7}', r#"\models"#);
    map.insert('\u{22a8}', r#"\vDash"#);
    //
    map.insert('\u{22c0}', r#"\bigwedge"#);
    map.insert('\u{22c1}', r#"\bigvee"#);
    map.insert('\u{22c2}', r#"\bigcap"#);
    map.insert('\u{22c3}', r#"\bigcup"#);
    map.insert('\u{22c4}', r#"\diamond"#);
    map.insert('\u{22c5}', r#"\cdot"#);
    map.insert('\u{22c6}', r#"\star"#);
    //
    map.insert('\u{22ce}', r#"\curlyvee"#);
    map.insert('\u{22cf}', r#"\curlywedge"#);

    map
  };


  /// ### UTF8_TEXT_TO_LATEX_MAP
  ///
  /// A mapping of characters that LaTeX does not recognize as text to escaped versions of them.
  /// This allows the parser to transform any plain text node contents into LaTeX-compatible strings.
  static ref UTF8_TEXT_TO_LATEX_MAP: HashMap<char, &'static str> = {

    let mut map = HashMap::new();

    map.insert('_', r#"\_"#);
    // map.insert('@', r#"\@ "#);
    map.insert('#', r#"\#"#);

    map
  };
}

