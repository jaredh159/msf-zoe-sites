use rocket::response::Responder;

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

pub fn head(page: Option<&str>) -> String {
  include_str!("assets/html/head.html").replace(
    "{%page_title%}",
    &format!(
      "{}Market Street Fellowship",
      page.map_or("".to_string(), |p| format!("{} | ", p))
    ),
  )
}
