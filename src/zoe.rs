use crate::component;
use crate::internal::*;

pub fn get() -> Html {
  let html = include_str!("assets/html/index.es.html")
    .replace("{%head%}", &html::head(None, Lang::Spanish))
    .replace("{%hero%}", &component::Hero::new(Lang::Spanish).html())
    .replace(
      "{%audios%}",
      &audio_groups()
        .into_iter()
        .map(|group| group.html())
        .collect::<Vec<_>>()
        .join("\n"),
    )
    .replace(
      "{%links%}",
      &links()
        .into_iter()
        .map(|link| link.html())
        .collect::<Vec<_>>()
        .join("\n"),
    );

  Html::new(&html)
}

pub fn links() -> Vec<component::Link> {
  vec![
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
  ]
}

pub fn audio_groups() -> Vec<component::AudioGroup> {
  vec![
    component::AudioGroup::new(
      "El Evangelio",
      vec![
        Teaching {
          id: 1,
          title: "Pt. 1 - El Poder de Dios".to_string(),
          filename: "El-Evangelio-pt-1.mp3".to_string(),
          duration: 2397,
          date: "2023-07-14 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/el-evangelio-pt-1"),
          ..Default::default()
        },
        Teaching {
          id: 2,
          title: "Pt. 2 - Salir del Egipto Interno".to_string(),
          filename: "El-Evangelio-pt-2.mp3".to_string(),
          duration: 1698,
          date: "2023-07-21 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/el-evangelio-pt-2"),
          ..Default::default()
        },
        Teaching {
          id: 3,
          title: "Pt. 3 - ¿Por Qué No Ha Cambiado Mi Corazón?".to_string(),
          filename: "El-Evangelio-pt-3.mp3".to_string(),
          duration: 2700,
          date: "2023-07-28 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/el-evangelio-pt-3"),
          ..Default::default()
        },
      ],
    ),
    component::AudioGroup::new(
      "La Ley y La Gracia",
      vec![
        Teaching {
          id: 4,
          title: "Pt. 1 - Cómo Entender la Ley".to_string(),
          filename: "La-Ley-y-La-Gracia-pt-1.mp3".to_string(),
          duration: 2150,
          date: "2023-08-25 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/la-ley-y-la-gracia-pt-1"),
          ..Default::default()
        },
        Teaching {
          id: 5,
          title: "Pt. 2 - La Verdadera Libertad de la Ley".to_string(),
          filename: "La-Ley-y-La-Gracia-pt-2.mp3".to_string(),
          duration: 2140,
          date: "2023-09-12 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/ley-y-gracia-pt-2"),
          ..Default::default()
        },
        Teaching {
          id: 6,
          title: "Pt. 3 - Cómo Entender la Gracia".to_string(),
          filename: "La-Ley-y-La-Gracia-pt-3.mp3".to_string(),
          duration: 2177,
          date: "2023-09-28 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/ley-y-gracia-pt-3"),
          ..Default::default()
        },
        Teaching {
          id: 7,
          title: "Pt. 4 - La Obra de la Gracia en el Corazón".to_string(),
          filename: "La-Ley-y-La-Gracia-pt-4.mp3".to_string(),
          duration: 2356,
          date: "2023-10-23 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/ley-y-gracia-pt-4"),
          ..Default::default()
        },
        Teaching {
          id: 8,
          title: "Pt. 5 - La Gracia Debería Reinar".to_string(),
          filename: "La-Ley-y-La-Gracia-pt-5.mp3".to_string(),
          duration: 2266,
          date: "2023-11-03 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/ley-y-gracia-pt-5"),
          ..Default::default()
        },
      ],
    ),
    component::AudioGroup::new(
      "Familiaridad con el Espíritu",
      vec![
        Teaching {
          id: 9,
          title: "Pt. 1 - Una división interior".to_string(),
          filename: "Familiaridad-con-el-Espiritu-pt-1.mp3".to_string(),
          duration: 2073,
          date: "2024-12-10 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/una-division-interior"),
          ..Default::default()
        },
        Teaching {
          id: 10,
          title: "Pt. 2 - Dejando al instante las redes, le siguieron".to_string(),
          filename: "Familiaridad-con-el-Espiritu-pt-2.mp3".to_string(),
          duration: 1929,
          date: "2024-12-13 12:00:00".to_string(),
          text_url: Some(
            "https://hender.blog/ensenanzas/dejando-al-instante-las-redes-le-siguieron",
          ),
          ..Default::default()
        },
        Teaching {
          id: 11,
          title: "Pt. 3 - El Desierto".to_string(),
          filename: "Familiaridad-con-el-Espiritu-pt-3.mp3".to_string(),
          duration: 1844,
          date: "2024-12-23 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/el-desierto"),
          ..Default::default()
        },
      ],
    ),
    component::AudioGroup::new(
      "El Reino de Dios",
      vec![
        Teaching {
          id: 12,
          title: "Pt. 1 - El Reino de Dios Se Ha Acercado".to_string(),
          filename: "El-Reino-de-Dios-pt-1.mp3".to_string(),
          duration: 2673,
          date: "2024-04-22 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/el-reino-de-dios-se-ha-acercado"),
          ..Default::default()
        },
        Teaching {
          id: 13,
          title: "Pt. 2 - La Venida del Rey".to_string(),
          filename: "El-Reino-de-Dios-pt-2.mp3".to_string(),
          duration: 2438,
          date: "2024-04-30 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/la-venida-del-rey"),
          ..Default::default()
        },
        Teaching {
          id: 14,
          title: "Pt. 3 - Encontrar y Seguir al Rey".to_string(),
          filename: "El-Reino-de-Dios-pt-3.mp3".to_string(),
          duration: 2479,
          date: "2024-05-06 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/encontrar-y-seguir-al-rey"),
          ..Default::default()
        },
      ],
    ),
    component::AudioGroup::new(
      "El Nuevo Pacto",
      vec![
        Teaching {
          id: 15,
          title: "Pt. 1 - El Pacto es una Vida".to_string(),
          filename: "El-Nuevo-Pacto-pt-1.mp3".to_string(),
          duration: 2111,
          date: "2024-06-26 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/el-pacto-es-una-vida"),
          ..Default::default()
        },
        Teaching {
          id: 16,
          title: "Pt. 2 - Caminar con Dios".to_string(),
          filename: "El-Nuevo-Pacto-pt-2.mp3".to_string(),
          duration: 2291,
          date: "2024-07-17 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/caminar-con-dios"),
          ..Default::default()
        },
        Teaching {
          id: 17,
          title: "Pt. 3 - Un Corazón Circuncidado".to_string(),
          filename: "El-Nuevo-Pacto-pt-3.mp3".to_string(),
          duration: 2273,
          date: "2024-08-06 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/un-corazon-circuncidado"),
          ..Default::default()
        },
        Teaching {
          id: 18,
          title: "Pt. 4 - Guardar el Pacto".to_string(),
          filename: "El-Nuevo-Pacto-pt-4.mp3".to_string(),
          duration: 2470,
          date: "2024-08-14 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/guardar-el-pacto"),
          ..Default::default()
        },
      ],
    ),
    component::AudioGroup::new(
      "Andar en el Espíritu",
      vec![
        Teaching {
          id: 19,
          title: "Pt. 1 - Dos Guías en Cada Hombre".to_string(),
          filename: "Andar-en-el-Espiritu-pt-1.mp3".to_string(),
          duration: 1631,
          date: "2023-06-05 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/andar-en-el-espiritu-pt-1"),
          ..Default::default()
        },
        Teaching {
          id: 20,
          title: "Pt. 2 - ¿Hacia Dónde Nos Guía el Espíritu?".to_string(),
          filename: "Andar-en-el-Espiritu-pt-2.mp3".to_string(),
          duration: 1560,
          date: "2023-07-06 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/andar-en-el-espiritu-pt-2"),
          ..Default::default()
        },
      ],
    ),
    component::AudioGroup::new(
      "Otros Mensajes",
      vec![
        Teaching {
          id: 21,
          title: "La Cruz de Cada Día".to_string(),
          filename: "La-Cruz-de-Cada-Dia.mp3".to_string(),
          duration: 1381,
          date: "2023-03-29 12:00:00".to_string(),
          text_url: Some("https://hender.blog/ensenanzas/la-cruz-de-cada-dia"),
          ..Default::default()
        },
        Teaching {
          id: 22,
          title: "La Naturaleza de la Obra Consumada de Cristo".to_string(),
          filename: "La-Obra-Consumada.mp3".to_string(),
          duration: 3399,
          date: "2023-02-13 12:00:00".to_string(),
          text_url: Some(
            "https://hender.blog/ensenanzas/la-naturaleza-de-la-obra-consumada-de-cristo",
          ),
          ..Default::default()
        },
      ],
    ),
  ]
}
