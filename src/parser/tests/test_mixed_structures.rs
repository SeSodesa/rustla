/*!
A submodule for testing mixed structures.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

#[cfg(test)]
#[test]
fn mixed_nested_lists_01() {
    let src = String::from(
        "
(i) * List item 1
      of a nested bullet list within
      an enumerated list...

    * Nested list item 2

      b) Nested enuemrated list in a nested bullet list

    Second paragraph of list item i.

(ii) List item 2 of the parent list.

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::EnumeratedListItem { .. } => (),
        _ => panic!(),
    }

    match doctree

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletList { .. } => (),
        _ => panic!(),
    }

    match doctree

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { .. } => (),
        _ => panic!(),
    }

    match doctree

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()

        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { .. } => (),
        _ => panic!(),
    }

    match doctree

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()

        .shared_child(1).unwrap()

        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!(),
    }

    match doctree

        .shared_child(0).unwrap()

        .shared_child(0).unwrap()

        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::EnumeratedListItem { .. } => (),
        _ => panic!(),
    }
}


#[test]
fn binomial_distribution_01 () {
    // Taken from https://www.overleaf.com/project/5cd12f9f3dad7b5b9f18d75a
    let src = String::from(
r#"
=============
Binomijakauma
=============

Palataan sitten tutkimaan tärkeimpiä diskreettejä ja jatkuvia todennäköisyysjakaumia.
Niistä ensimmäinen muodostuu seuraavan *Bernoullin kokeen* toistona.
Oletetaan, että satunnaiskokeen tulosvaihtoehdot (koodattuna) ovat :math:`0` ja :math:`1`,
eli tarkastelun kohteena oleva tapahtuma joko ei realisoidu tai realisoituu.
Ensimmäistä vaihtoehtoa kutsutaan myös epäonnistumiseksi ja jälkimmäistä onnistumiseksi.
Kiinnitetään onnistumisen todennäköisyydeksi :math:`p`, jolloin epäonnistumisen todennäköisyys on :math:`1 - p`.

.. maaritelma::

  Diskreetti satunnaismuuttuja :math:`X` noudattaa *Bernoullin jakaumaa* (Bernoulli distribution) parametrilla :math:`p`, :math:`X \sim \Ber(p)`, jos sen otosavaruus :math:`\Omega = \{0, 1\}` ja tiheysfunktio

  .. math::

    f(x) =
    \begin{cases}
    p, & \text{kun } x = 1 \\ 1 - p, & \text{kun } x = 0.
    \end{cases}

Bernoullin jakauman kuvaama satunnaiskoe voidaan yleistää toistamalla sitä :math:`n` kertaa siten,
että jokainen toisto on toisista riippumaton.
Onnistumisien lukumäärä tässä :math:`n`-toistokokeessa on uusi diskreetti satunnaismuuttuja :math:`X`,
jonka mahdolliset arvot ovat kokonaislukuja :math:`0,1,\ldots,n`. Siihen liittyvät alkeistapaukset ovat nollista ja
ykkösistä koostuvia jonoja, joissa on :math:`x` kappaletta onnistumisia ja :math:`n - x` kappaletta epäonnistumisia
jossakin järjestyksessä. Koska onnistumisen todennäköisyys on :math:`p` ja toistot ovat riippumattomia toisistaan,
yksittäinen alkeistapaus realisoituu todennäköisyydellä :math:`p^{x}(1 - p)^{n - x}`.
Vaihtoehtoja alkeistapahtumiksi, joihin liittyy :math:`x` onnistumista, on :math:`\binom{n}{x}` erilaista, joten

.. math::

  P(X=x)=\binom{n}{x}p^x(1-p)^{n-x}.

.. maaritelma::

  Diskreetti satunnaismuuttuja :math:`X` noudattaa *binomijakaumaa* (binomial distribution)
  parametrein :math:`n` ja :math:`p`,:math:`X\sim\Bin(n,p)`, jos sen otosavaruus

  .. math::

    \Omega=\{0,1,\ldots,n\}

  ja tiheysfunktio

  .. math::

    f(x)=b(x; n, p)=\binom{n}{x}p^x(1-p)^{n-x},\qquad\text{kun }x \in \Omega.

.. image:: kuvat/kuva29bin1.*
  :width: 50%
  :align: center

.. image:: kuvat/kuva29bin2.*
  :width: 50%
  :align: center
"#,
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Section {title_text, level, line_style} => {
            assert_eq!(title_text, "Binomijakauma");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::UnknownDirective {directive_name, argument, options, ..} => {
            assert_eq!(directive_name, "maaritelma");
            assert_eq!(argument, "");
            assert!(options.is_empty());
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::MathBlock { math_block, .. } => {
            assert_eq!(math_block, "f(x) =\n\
            \\begin{cases}\n\
            p, & \\text{kun } x = 1 \\\\ 1 - p, & \\text{kun } x = 0.\n\
            \\end{cases}");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(3).unwrap().shared_data() {
        TreeNodeType::MathBlock { math_block, name, class } => {
            assert_eq!(math_block, "P(X=x)=\\binom{n}{x}p^x(1-p)^{n-x}.");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(3).unwrap().shared_data() {
        TreeNodeType::MathBlock { math_block, name, class } => {
            assert_eq!(math_block, "P(X=x)=\\binom{n}{x}p^x(1-p)^{n-x}.");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(4).unwrap().shared_data() {
        TreeNodeType::UnknownDirective { directive_name, argument, options, .. } => {
            assert_eq!(directive_name, "maaritelma");
            assert_eq!(argument, "");
            assert!(options.is_empty());
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(4).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(4).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::MathBlock { math_block, .. } => {
            assert_eq!(math_block, "\\Omega=\\{0,1,\\ldots,n\\}");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(4).unwrap()
        .shared_child(2).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(4).unwrap()
        .shared_child(3).unwrap().shared_data() {
        TreeNodeType::MathBlock { math_block, .. } => {
            assert_eq!(math_block, "f(x)=b(x; n, p)=\\binom{n}{x}p^x(1-p)^{n-x},\\qquad\\text{kun }x \\in \\Omega.");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(5).unwrap().shared_data() {
        TreeNodeType::Image { uri, alt, height, width, scale, align, target, name, class, inline } => {
            assert_eq!(uri, "kuvat/kuva29bin1.*");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(0).unwrap()
        .shared_child(6).unwrap().shared_data() {
        TreeNodeType::Image { uri, alt, height, width, scale, align, target, name, class, inline } => {
            assert_eq!(uri, "kuvat/kuva29bin2.*");
        }
        _ => panic!()
    }
}
