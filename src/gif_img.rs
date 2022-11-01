use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct GifImgProps {
  pub gif: String,
  pub img: String,
  #[prop_or_default]
  pub alt: String,
  #[prop_or_default]
  pub gifalt: String,
  #[prop_or_default]
  pub width: Option<String>,
  #[prop_or_default]
  pub height: Option<String>,
}

#[function_component(GifImg)]
pub fn gif_img(props: &GifImgProps) -> Html {
  let is_clicked = use_state(|| false);
  let onclick = {
    let is_clicked = is_clicked.clone();
    Callback::from(move |_| is_clicked.set(!*is_clicked))
  };

  html!{
    <div class="container">
      if *is_clicked {
        <img 
          src={props.gif.clone()} 
          alt={props.gifalt.clone()}
          width={props.width.clone()}
          height={props.height.clone()}
          {onclick}
        />
      } else{
        <img 
          src={props.img.clone()}
          alt={props.alt.clone()}
          width={props.width.clone()}
          height={props.height.clone()}
          {onclick}
        />
      }
    </div>
  }
}