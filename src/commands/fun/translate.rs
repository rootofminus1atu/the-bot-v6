use poise::CreateReply;
use serde::Deserialize;
use titlecase::titlecase;
use crate::{Context, Error};
use poise::serenity_prelude::{futures, Color, CreateEmbed};
use poise::futures_util::StreamExt;


/// Translate a message
/// 
/// Run this command to translate a message from one language to another. Use the slash command for a list of available langauges.
#[poise::command(slash_command, prefix_command, category = "Fun")]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "Translate from language..."]
    #[autocomplete = "autocomplete_language"]
    from: String,
    #[description = "...to another language"]
    #[autocomplete = "autocomplete_language"]
    to: String,
    #[description = "The message you want translated"]
    message: String
) -> Result<(), Error> {
    // let from = from.to_lowercase();
    // let to = to.to_lowercase();

    // let from_code = find_lang_code(&from).ok_or(format!("Didn't recognize `{}` as a language", from))?;
    // let to_code = find_lang_code(&to).ok_or(format!("Didn't recognize `{}` as a language", to))?;
    let from  = Language::parse(&from)?;
    let to  = Language::parse(&to)?;

    let translated_message = get_translation(&from, &to, &message, &ctx.data().client, &ctx.data().translation_key).await?;
    
    ctx.send(CreateReply::default()
        .embed(CreateEmbed::new()
            .title("Translation")
            .color(Color::BLURPLE)
            .field(
                format!("From {}:", titlecase(&from.name)), 
                message, 
                true)
            .field(
                format!("To {}:", titlecase(&to.name)),
                titlecase(&translated_message),
                true,)
        )
    ).await?;

    Ok(())
}

async fn autocomplete_language<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl futures::Stream<Item = String> + 'a {
    let partial_lowercase = partial.to_lowercase();

    let (langs, _): (Vec<_>, Vec<_>) = LANG_CODES.iter().cloned().unzip();

    futures::stream::iter(langs)
        .filter(move |lang| futures::future::ready(lang.contains(&partial_lowercase)))
        .map(|lang| titlecase(lang))
        .map(|lang| lang.to_string())
}



async fn get_translation(from: &Language, to: &Language, message: &str, client: &reqwest::Client, key: &str) -> Result<String, Error> {
    let url = "https://translated-mymemory---translation-memory.p.rapidapi.com/get";

    let querystring = [
        ("langpair", format!("{}|{}", &from.code, &to.code)),
        ("q", message.to_string()),
        ("mt", "1".to_string()),
        ("onlyprivate", "0".to_string()),
        ("de", "a@b.c".to_string()),
    ];

    let json = client
        .get(url)
        .query(&querystring)
        .header("X-RapidAPI-Key", key)
        .header("X-RapidAPI-Host","translated-mymemory---translation-memory.p.rapidapi.com")
        .send()
        .await?
        .json::<TranslationResponse>()
        .await?;

    let translated_text = json.response_data.translated_text;

    Ok(translated_text)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationResponse {
    response_data: ResponseData
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    translated_text: String
}



pub struct Language {
    pub name: String,
    pub code: String
}

impl Language {
    pub fn parse(lang_str: &str) -> Result<Self, Error> {
        let code = find_lang_code(&lang_str.to_lowercase())
            .ok_or(format!("Didn't recognize `{}` as a language", lang_str))?
            .to_string();

        Ok(Self { name: lang_str.to_string(), code })
    }
}

fn find_lang_code(target: &str) -> Option<&str> {
    LANG_CODES.iter()
        .find(|(lang_name, _)| lang_name == &target)
        .map(|elem| elem.1)
}

const LANG_CODES: &[(&str, &str)] = &[
    ("amharic", "am-ET"),
    ("arabic", "ar-SA"),
    ("bielarus", "be-BY"),
    ("bemba", "bem-ZM"),
    ("bislama", "bi-VU"),
    ("bajan", "bjs-BB"),
    ("bengali", "bn-IN"),
    ("tibetan", "bo-CN"),
    ("breton", "br-FR"),
    ("bosnian", "bs-BA"),
    ("catalan", "ca-ES"),
    ("coptic", "cop-EG"),
    ("czech", "cs-CZ"),
    ("welsh", "cy-GB"),
    ("danish", "da-DK"),
    ("dzongkha", "dz-BT"),
    ("german", "de-DE"),
    ("maldivian", "dv-MV"),
    ("greek", "el-GR"),
    ("english", "en-GB"),
    ("spanish", "es-ES"),
    ("estonian", "et-EE"),
    ("basque", "eu-ES"),
    ("persian", "fa-IR"),
    ("finnish", "fi-FI"),
    ("fanagalo", "fn-FNG"),
    ("faroese", "fo-FO"),
    ("french", "fr-FR"),
    ("galician", "gl-ES"),
    ("gujarati", "gu-IN"),
    ("hausa", "ha-NE"),
    ("hebrew", "he-IL"),
    ("hindi", "hi-IN"),
    ("croatian", "hr-HR"),
    ("hungarian", "hu-HU"),
    ("indonesian", "id-ID"),
    ("icelandic", "is-IS"),
    ("italian", "it-IT"),
    ("japanese", "ja-JP"),
    ("kazakh", "kk-KZ"),
    ("khmer", "km-KM"),
    ("kannada", "kn-IN"),
    ("korean", "ko-KR"),
    ("kurdish", "ku-TR"),
    ("kyrgyz", "ky-KG"),
    ("latin", "la-VA"),
    ("lao", "lo-LA"),
    ("latvian", "lv-LV"),
    ("mende", "men-SL"),
    ("malagasy", "mg-MG"),
    ("maori", "mi-NZ"),
    ("malay", "ms-MY"),
    ("maltese", "mt-MT"),
    ("burmese", "my-MM"),
    ("nepali", "ne-NP"),
    ("niuean", "niu-NU"),
    ("dutch", "nl-NL"),
    ("norwegian", "no-NO"),
    ("nyanja", "ny-MW"),
    ("pakistani", "ur-PK"),
    ("palauan", "pau-PW"),
    ("panjabi", "pa-IN"),
    ("pashto", "ps-PK"),
    ("pijin", "pis-SB"),
    ("polish", "pl-PL"),
    ("portuguese", "pt-PT"),
    ("kirundi", "rn-BI"),
    ("romanian", "ro-RO"),
    ("russian", "ru-RU"),
    ("sango", "sg-CF"),
    ("sinhala", "si-LK"),
    ("slovak", "sk-SK"),
    ("samoan", "sm-WS"),
    ("shona", "sn-ZW"),
    ("somali", "so-SO"),
    ("albanian", "sq-AL"),
    ("serbian", "sr-RS"),
    ("swedish", "sv-SE"),
    ("swahili", "sw-SZ"),
    ("tamil", "ta-LK"),
    ("telugu", "te-IN"),
    ("tetum", "tet-TL"),
    ("tajik", "tg-TJ"),
    ("thai", "th-TH"),
    ("tigrinya", "ti-TI"),
    ("turkmen", "tk-TM"),
    ("tagalog", "tl-PH"),
    ("tswana", "tn-BW"),
    ("tongan", "to-TO"),
    ("turkish", "tr-TR"),
    ("ukrainian", "uk-UA"),
    ("uzbek", "uz-UZ"),
    ("vietnamese", "vi-VN"),
    ("wolof", "wo-SN"),
    ("xhosa", "xh-ZA"),
    ("yiddish", "yi-YD"),
    ("zulu", "zu-ZA"),
];
