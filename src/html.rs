use crate::internal::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Lang {
  English,
  Spanish,
}

#[derive(Responder)]
#[response(content_type = "text/html")]
pub struct Html {
  content: String,
}

impl Html {
  pub fn new(content: &str) -> Self {
    let mut html = Self { content: content.to_string() };
    if cfg!(debug_assertions) {
      html
        .content
        .push_str(include_str!("assets/html/refresh.html"));
    } else {
      html.content = html.content.replace(
        "</body>",
        "<script>
        (function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
        (i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o),
        m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
        })(window,document,'script','https://www.google-analytics.com/analytics.js','ga');
        ga('create', 'UA-2843792-2', 'auto');
        ga('send', 'pageview');
        </script></body>",
      );
    }
    html
  }
}

pub fn head(page: Option<&str>, language: Lang) -> String {
  let (site_name, description) = match language {
    Lang::English => (
      "Market Street Fellowship",
      "Market Street Fellowship is a non-denominational Christian church located in Wadsworth, Ohio, and committed to a whole-hearted following of Christ in the ancient path of the daily cross. We believe that the kingdom of God is not in traditions and words, but in power; a power (called grace) that overcomes sin, self, and the world, and experientially transforms the heart into the image and nature of Christ.",
    ),
    Lang::Spanish => (
      "Zoe Costa Rica",
      "Zoe Costa Rica es un sitio web dedicado a la entrega absoluta del corazón a Jesucristo en el camino antiguo de la cruz diaria. Creemos que el reino de Dios no consiste en tradiciones ni palabras, sino en poder; un poder (llamado gracia) que vence el pecado, el yo y el mundo, y transforma genuinamente el corazón a la imagen y naturaleza de Cristo.",
    ),
  };

  include_str!("assets/html/head.html")
    .replace(
      "{%page_title%}",
      &format!(
        "{}{}",
        page.map_or("".to_string(), |p| format!("{} | ", p)),
        site_name
      ),
    )
    .replace("{%description%}", description)
}
