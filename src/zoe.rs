use crate::component;
use crate::date;
use crate::internal::*;

pub fn get() -> Html {
  let teachings = Teaching::load_most_recent(5);
  let links = vec![
    component::Link::new(
      "Biblioteca de los Amigos",
      "https://www.bibliotecadelosamigos.org",
      "Los creadores de Zoe Costa Rica se han visto tan impactados por los escritos de los primeros (¡no los modernos!) Cuáqueros (1650-1800), que creamos un sitio web donde multitud de sus libros han sido cuidadosamente traducidos al español y están disponibles de manera gratuita, en varios formatos de texto y audio.",
    ),
    component::Link::new(
      "La Senda Antigua",
      "https://hender.blog",
      "Más enseñanzas y publicaciones (en texto y audio) de Jason Henderson sobre una variedad de temas diferentes.",
    ),
    component::Link::new(
      "Gertrude",
      "https://gertrude.app",
      "Jared Henderson y Miciah Henderson han pasado años desarrollando lo que creemos que es el software de control parental más seguro que existe para computadoras Apple. También existe una aplicación correspondiente para iPhone, que corrige algunas de las deficiencias que la función Tiempo en Pantalla de Apple no alcanza a proteger.",
    ),
  ];

  let html = include_str!("assets/html/index.es.html")
    .replace("{%head%}", &html::head(None, Language::Spanish))
    .replace(
      "{%audios%}",
      &teachings
        .into_iter()
        .map(|t| component::Audio { teaching: t }.html())
        .collect::<Vec<_>>()
        .join("\n"),
    )
    .replace(
      "{%links%}",
      &links
        .into_iter()
        .map(|l| l.html())
        .collect::<Vec<_>>()
        .join("\n"),
    );

  Html::new(&html)
}

