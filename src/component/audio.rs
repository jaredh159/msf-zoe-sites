use crate::internal::*;

#[derive(Debug, Clone)]
pub struct Audio {
  pub teaching: Teaching,
}

impl Audio {
  pub fn html(&self) -> String {
    let template = include_str!("audio.html");
    let (texto_link_desktop, texto_link_mobile) = if let Some(text_url) = &self.teaching.text_url {
      let desktop_link = format!(
        r#"<a href="{}" target="_blank" rel="noopener noreferrer" class="hidden sm:inline text-green-700 hover:text-green-800 underline font-medium">texto</a>"#,
        text_url
      );
      let mobile_link = format!(
        r#"<span class="mx-2">•</span><a href="{}" target="_blank" rel="noopener noreferrer" class="text-green-700 hover:text-green-800 underline font-medium sm:hidden">texto</a>"#,
        text_url
      );
      (desktop_link, mobile_link)
    } else {
      (String::new(), String::new())
    };

    template
      .replace("{%title%}", &self.teaching.title)
      .replace("{%speaker%}", &self.teaching.speaker)
      .replace("{%date%}", &self.teaching.short_date())
      .replace("{%url%}", &self.teaching.url())
      .replace("{%duration%}", &self.teaching.human_duration())
      .replace("{%texto_link_desktop%}", &texto_link_desktop)
      .replace("{%texto_link_mobile%}", &texto_link_mobile)
  }
}
