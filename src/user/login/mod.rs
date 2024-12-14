use crate::core::{
    html::{children::text, div, Elem},
    http::response_writer::ResponseWriter,
    ui::drawer::Drawer,
};

impl ResponseWriter {
    pub async fn send_must_login_first(&mut self) -> Result<(), std::io::Error> {
        println!("You must login in first");
        self.send_fragment(view_must_login_drawer()).await?;

        println!("You must login in first");

        Ok(())
    }
}

fn view_must_login_drawer() -> Elem {
    Drawer::default()
        .model_open("signal_drawer_open")
        .initial_open(true)
        .on_close("signal_drawer_open.value = false")
        .content(
            div()
                .class("w-full h-full p-6")
                .child(text("You must login in first"))
                .child(div().class("flex items-center gap-6")),
        )
        .view()
}
