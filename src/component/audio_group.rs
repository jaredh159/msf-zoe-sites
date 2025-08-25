use crate::internal::*;

#[derive(Debug, Clone)]
pub struct AudioGroup {
  title: String,
  teachings: Vec<Teaching>,
}

impl AudioGroup {
  pub fn new(title: &str, teachings: Vec<Teaching>) -> Self {
    Self {
      title: title.to_string(),
      teachings,
    }
  }

  pub fn html(&self) -> String {
    let teachings_html = self
      .teachings
      .iter()
      .map(|t| component::Audio { teaching: t.clone() }.html())
      .collect::<Vec<_>>()
      .join("\n");

    format!(
      r#"<div class="mb-6">
  <h3 class="text-lg font-semibold text-green-800 mb-4">
    {}
  </h3>
  <div class="grid grid-cols-1 gap-3">
    {}
  </div>
</div>"#,
      self.title, teachings_html
    )
  }
}