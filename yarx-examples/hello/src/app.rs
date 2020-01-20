use meet::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
  }
  
  pub enum Msg {
      Click,
  }
  
  impl Component for Model {
      type Message = Msg;
      type Properties = ();
  
      fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
          Model { link }
      }
  
      fn update(&mut self, msg: Self::Message) -> ShouldRender {
          match msg {
              Msg::Click => true,
          }
      }
  
      fn view(&self) -> Html {
          // AFTER: Callbacks need to be explicitly created now
          let onclick = self.link.callback(|_| Msg::Click);
          html! {
              <button onclick=onclick>{ "Click me!" }</button>
          }
      }
  }