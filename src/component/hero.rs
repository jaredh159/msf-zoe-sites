use crate::internal::*;

#[derive(Debug, Clone)]
pub struct Hero {
  pub language: Lang,
}

impl Hero {
  pub fn new(language: Lang) -> Self {
    Self { language }
  }

  pub fn html(&self) -> String {
    let (title, description_p1, description_p2) = match self.language {
      Lang::English => (
        "Market Street Fellowship",
        "Market Street Fellowship is a non-denominational Christian church located in Wadsworth, Ohio, and committed to a whole-hearted following of Christ in the “ancient path” of the daily cross. We believe that the kingdom of God is not in traditions and words, but in power; a power (called grace) that overcomes sin, self, and the world, and experientially transforms the heart into the image and nature of Christ.",
        "Our desire is to walk together in His light, life and love, to surrender our own will to His in all things, and to grow up together into Him who is the Head, even Christ.",
      ),
      Lang::Spanish => (
        "Zoe Costa Rica",
        "Zoe Costa Rica es un sitio web dedicado a la entrega absoluta del corazón a Jesucristo en el “camino antiguo” de la cruz diaria. Creemos que el reino de Dios no consiste en tradiciones ni palabras, sino en poder; un poder (llamado gracia) que vence el pecado, el yo y el mundo, y transforma genuinamente el corazón a la imagen y naturaleza de Cristo.",
        "Nuestro deseo es caminar juntos en Su luz, vida y amor, rendir nuestra propia voluntad a la Suya en todas las cosas, y crecer juntos en Aquel que es la Cabeza, esto es, Cristo.",
      ),
    };

    let template = include_str!("hero.html");
    template
      .replace("{%title%}", title)
      .replace("{%description_p1%}", description_p1)
      .replace("{%description_p2%}", description_p2)
  }
}
