use tide::prelude::*;
use tide::{http::mime, Request, Response, Result};

use crate::util::encryption;

use crate::wiring::ServerWiring;
use tinytemplate::TinyTemplate;

use askama::Template; // bring trait in scope

#[derive(Debug)]
enum MediaType {
    Image,
    Text,
    Audio,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
trait MediaRenderer {
    fn render_json(&self) -> String;
}

#[derive(Serialize, Deserialize)]
struct ImageMedia {
    url: String,
}

impl MediaRenderer for ImageMedia {
    fn render_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct TextMedia {
    body: String,
}

impl MediaRenderer for TextMedia {
    fn render_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct AudioMedia {
    title: String,
    duration: i32,
    khz: i32,
    kbps: i32,
    url: String,
}

impl MediaRenderer for AudioMedia {
    fn render_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

struct MediaNodeHtml {
    innerHTML: String,
}

struct MediaNodeBundle {
    media_type: MediaType,
    slug: String,
    template: String,
    renderer: Box<dyn MediaRenderer>,
}

impl MediaNodeBundle {
    fn text(slug: &str, template: &str, txt: TextMedia) -> Self {
        MediaNodeBundle {
            media_type: MediaType::Text,
            slug: slug.to_owned(),
            template: template.to_owned(),
            renderer: Box::new(txt),
        }
    }

    fn audio(slug: &str, template: &str, audio: AudioMedia) -> Self {
        MediaNodeBundle {
            media_type: MediaType::Audio,
            slug: slug.to_owned(),
            template: template.to_owned(),
            renderer: Box::new(audio),
        }
    }

    fn img(slug: &str, template: &str, img: ImageMedia) -> Self {
        MediaNodeBundle {
            media_type: MediaType::Image,
            slug: slug.to_owned(),
            template: template.to_owned(),
            renderer: Box::new(img),
        }
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "media/list.html.j2")] // using the template in this path, relative
struct ListGetViewModel {
    media: Vec<MediaNodeHtml>,
}

#[derive(Template)] // this will generate the code...
#[template(path = "media/node.html.j2")] // using the template in this path, relative
struct MediaNodeViewModel {
    slug: String,
    medium: String,
    media: String,
}

#[derive(Serialize, Deserialize)]
struct MediaTemplateContext {
    media: String,
}

fn render_bundle(bundle: &MediaNodeBundle) -> MediaNodeHtml {
    let mut tt = TinyTemplate::new();

    tt.add_template(&bundle.slug, &bundle.template)
        .expect("hope I can add this template!");

    tt.set_default_formatter(&tinytemplate::format_unescaped);

    let media_json = bundle.renderer.render_json();
    let media_json_base64 = base64::encode(media_json);

    let media_context = MediaNodeViewModel {
        slug: bundle.slug.to_owned(),
        medium: bundle.media_type.to_string().to_ascii_lowercase(),
        media: media_json_base64,
    };

    let media_html = media_context
        .render()
        .expect("error rendering media node html");

    let context = MediaTemplateContext { media: media_html };

    MediaNodeHtml {
        innerHTML: tt
            .render(&bundle.slug, &context)
            .expect("error in rendering custom media template!"),
    }
}

struct FakeMediaDatabase {}

use uuid::Uuid;

impl FakeMediaDatabase {
    pub fn gen_slug() -> String {
        let id = Uuid::new_v4();
        format!("/media@{}", id.urn())
    }

    pub fn get_media() -> Vec<MediaNodeHtml> {
        let items = vec![
            MediaNodeBundle::img(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="w-96 inline-block">{media}</div>"#,
                ImageMedia {
                    url: String::from("https://cloud.bunny.holycharisma.com/luna_beam_fx.gif"),
                },
            ),
            
            MediaNodeBundle::text(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="w-72 text-left align-top text-xl pl-3 inline-block">{media}</div>"#,
                TextMedia {
                    body: String::from(
                        r#"Two artist-musician-lovers hailing from the world of techno-babble and blackberries, Mary and Tommy Charisma formed Holy Charisma to blaze a musical trail across psychedelic landscapes."#,
                    ),
                },
            ),

            MediaNodeBundle::text(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div>{media}</div>"#,
                TextMedia {
                    body: String::from(""),
                },
            ),

            MediaNodeBundle::img(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="inline-block">{media}</div>"#,
                ImageMedia {
                    url: String::from("https://cloud.bunny.holycharisma.com/uwu_sun.png"),
                },
            ),

            MediaNodeBundle::text(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="text-9xl pl-3 tracking-tighter inline-block">{media}</div>"#,
                TextMedia {
                    body: String::from("The goal is ananda for all."),
                },
            ),

            MediaNodeBundle::text(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="text-left align-top text-xl pl-3">{media}</div>"#,
                TextMedia {
                    body: String::from("A place where dreams go to blossom - where you can plant a little seed of hope and watch it grow into a beautiful garden."),
                },
            ),

            MediaNodeBundle::text(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="h-3">{media}</div>"#,
                TextMedia {
                    body: String::from(""),
                },
            ),


            MediaNodeBundle::text(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="w-96 text-left align-top text-3xl pl-3 inline-block">{media}</div>"#,
                TextMedia {
                    body: String::from(
                        "Through narratives carved out in grooves they get people dancing and connecting with each other - the musical ear worms inject the punk-friendly philosophy of the diggers and the dead - rolled up in an old spiritualist rag and ready to smoke."
                    )
                },
            ),

            MediaNodeBundle::img(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"<div class="inline-block">{media}</div>"#,
                ImageMedia {
                    url: String::from("https://cloud.bunny.holycharisma.com/party.png"),
                },
            ),

            MediaNodeBundle::audio(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"{media}"#,
                AudioMedia {
                    title: String::from("metanoia"),
                    url: String::from("https://cloud.bunny.holycharisma.com/metanoia.mp3"),
                    duration: 278,
                    khz: 48,
                    kbps: 192,
                },
            ),
            MediaNodeBundle::audio(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"{media}"#,
                AudioMedia {
                    title: String::from("untitled love loop"),
                    url: String::from(
                        "https://cloud.bunny.holycharisma.com/untitled-love-loop.mp3",
                    ),
                    duration: 15,
                    khz: 44,
                    kbps: 192,
                },
            ),
            MediaNodeBundle::audio(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"{media}"#,
                AudioMedia {
                    title: String::from("ready"),
                    url: String::from("https://cloud.bunny.holycharisma.com/ready.mp3"),
                    duration: 186,
                    khz: 44,
                    kbps: 320,
                },
            ),
            MediaNodeBundle::audio(
                FakeMediaDatabase::gen_slug().as_str(),
                r#"{media}"#,
                AudioMedia {
                    title: String::from("fly 2 infinity"),
                    url: String::from("https://cloud.bunny.holycharisma.com/fly_to_infinity.mp3"),
                    duration: 231,
                    khz: 44,
                    kbps: 320,
                },
            ),
        ];

        items.iter().map(|e| render_bundle(e)).collect()
    }
}

pub async fn get(req: Request<ServerWiring>) -> Result {
    let rendered_media: Vec<MediaNodeHtml> = FakeMediaDatabase::get_media();

    let view_context = ListGetViewModel {
        media: rendered_media,
    };

    let secrets: &encryption::SharedKeyring = req.ext().unwrap();

    let encrypted_body = secrets
        .encrypt_broadcast_emoji(&view_context.render().unwrap())
        .await
        .unwrap()
        .message;

    let response = Response::builder(200)
        .content_type(mime::HTML)
        .body_string(encrypted_body)
        .build();
    Ok(response)
}
