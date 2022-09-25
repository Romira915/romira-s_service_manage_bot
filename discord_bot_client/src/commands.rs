use serenity::{builder::CreateEmbed, utils::Colour};

pub mod activity;
pub mod ai;
pub mod ark;
pub mod conversation;
pub mod minecraft;
pub mod sdtd;
pub mod simple;
pub mod terraria;
pub mod valheim;

const SUCCESS_IMAGE_URL: &'static str =
    "https://i.pinimg.com/564x/c9/72/f0/c972f0909879d3ce4137c7140e26922c.jpg";
const FAILED_IMAGE_URL: &'static str =
    "https://p100k.jp/wp-content/uploads/2021/03/EI4vUVMUYAAZzj7-1024x905-1-1.jpg";

#[derive(Debug, Default)]
struct EmbedMessageBuilder {
    is_success: bool,
    message: String,
}

impl EmbedMessageBuilder {
    fn success(&mut self, success: bool) -> &mut Self {
        self.is_success = success;
        self
    }

    fn message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }

    fn build(&self) -> CreateEmbed {
        let (title, description, field_icon, color, image_url) = if self.is_success {
            (
                "Success!",
                "Success details",
                ":ok:",
                Colour::BLUE,
                SUCCESS_IMAGE_URL,
            )
        } else {
            (
                "Error!",
                "Error details",
                ":warning:",
                Colour::RED,
                FAILED_IMAGE_URL,
            )
        };

        let mut embed = CreateEmbed::default();
        embed
            .title(title)
            .description(description)
            .field(field_icon, &self.message, false)
            .colour(color)
            .image(image_url)
            .footer(|f| f.text(description));
        embed

        // create_massage.embed(|e| {
        //     e.title(title)
        //         .description(description)
        //         .field(field_icon, &self.message, false)
        //         .colour(color)
        //         .image(image_url)
        //         .footer(|f| f.text(description))
        // })
    }
}
