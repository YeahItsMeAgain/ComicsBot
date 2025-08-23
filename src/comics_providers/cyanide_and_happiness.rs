use crate::comics_providers::ComicsProviders;
use crate::db::{
    entities::{channel, channel_comics_provider_subscription, comics_provider},
    get_db,
};
use crate::store::get_store;
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use teloxide::{
    Bot,
    payloads::SendPhotoSetters,
    prelude::Requester,
    types::{ChatId, InputFile},
};
use tokio::time;

const EXPLOSM_LATEST_URL: &str = "http://explosm.net/comics/latest";
const EXPLOSM_NEXT_JS_COMICS_URL: &str =
    "https://explosm.net/_next/data/{next_js_build_id}/comics.json";
const EXPLOSM_LAST_COMIC_TIMESTAMP_KEY: &str = "explosm_last_comic_timestamp";

#[derive(Serialize, Deserialize, Debug)]
struct NextJsData {
    #[serde(alias = "buildId")]
    build_id: String,
}

/*
The next js comics.json data format is a dynamic beast:
{
    "pageProps": {
        "comicsArchiveData": {
            "2025": {
                "0": [{"author_name": .....}],
                "1": [{"author_name": .....}],
            },
            "2026": {
                ....
            }
        }
    }
}
*/
#[derive(Debug, Deserialize)]
struct ComicsResponseData {
    #[serde(alias = "pageProps")]
    props: PageProps,
}

#[derive(Debug, Deserialize)]
struct PageProps {
    #[serde(alias = "comicArchiveData")]
    comics_archive: HashMap<String, HashMap<String, Vec<ComicEntry>>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ComicEntry {
    author_name: Option<String>,
    file_static: Option<String>,
    publish_at: Option<String>,
    slug: Option<String>,
}

/*
    The steps to receive all the new comics:
    * GET http://explosm.net/comics/latest and extract the nextjs buildId from the html.
    * GET https://explosm.net/_next/data/{buildId}/comics.json.
    * Iterate over the json.
    * PROFIT!
*/
pub async fn notify_changes(bot: Bot) -> Result<()> {
    let next_js_build_id = get_next_js_build_id().await?;
    log::info!("Received next js build id: {next_js_build_id}");

    let comics_data =
        reqwest::get(EXPLOSM_NEXT_JS_COMICS_URL.replace("{next_js_build_id}", &next_js_build_id))
            .await?
            .json::<ComicsResponseData>()
            .await?;

    let mut last_comic_timestamp = get_last_comic_timestamp();

    log::info!(
        "Iterating over explosm comics, last comic date: {:?}",
        DateTime::from_timestamp(last_comic_timestamp, 0)
    );

    let comic_provider = get_comics_provider().await?;
    // Iterate over the comics in order, parse and send the new ones to the subscribed channels.
    for (_year, comics) in comics_data.props.comics_archive {
        for (_index, comics) in comics {
            for comic in comics {
                let (Some(comic_slug), Some(published_at), Some(comic_url), Some(comic_author)) = (
                    comic.slug,
                    comic.publish_at,
                    comic.file_static,
                    comic.author_name,
                ) else {
                    continue;
                };

                let published_at =
                    NaiveDateTime::parse_from_str(&published_at, "%Y-%m-%d %H:%M:%S")?;
                let published_at_timestamp = published_at.and_utc().timestamp();
                if published_at_timestamp < last_comic_timestamp {
                    continue;
                }
                last_comic_timestamp = published_at_timestamp;

                let Ok(comic_url) = Url::parse(&comic_url) else {
                    log::error!("Invalid comic url: {comic_url}");
                    continue;
                };

                for subscription in comic_provider
                    .find_related(channel_comics_provider_subscription::Entity)
                    .all(get_db())
                    .await?
                {
                    let channel = subscription
                        .find_related(channel::Entity)
                        .one(get_db())
                        .await?
                        .unwrap();
                    let chat_id = ChatId(channel.tgid);
                    log::info!(
                        "Notifying {channel:?} with new comic: {}",
                        comic_url.clone()
                    );

                    if let Err(e) = send_new_comic(
                        &bot,
                        chat_id,
                        comic_url.clone(),
                        comic_slug.clone(),
                        published_at,
                        comic_author.clone(),
                    )
                    .await
                    {
                        log::error!("{e}");
                    }
                    // Maybe will us not get banned :]
                    time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    get_store()
        .set(
            EXPLOSM_LAST_COMIC_TIMESTAMP_KEY,
            last_comic_timestamp.to_string(),
        )
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to save last explosm comic timestamp: {last_comic_timestamp}, {e}"
            )
        })?;
    Ok(())
}

async fn get_comics_provider() -> anyhow::Result<comics_provider::Model> {
    Ok(comics_provider::Entity::find()
        .filter(comics_provider::Column::Name.eq(ComicsProviders::CyanideAndHappiness.to_string()))
        .one(get_db())
        .await?
        .unwrap())
}

fn get_last_comic_timestamp() -> i64 {
    get_store()
        .get(EXPLOSM_LAST_COMIC_TIMESTAMP_KEY)
        .unwrap_or(Utc::now().timestamp().to_string())
        .parse::<i64>()
        .unwrap()
}

async fn send_new_comic(
    bot: &Bot,
    chat_id: ChatId,
    comic_url: Url,
    comic_slug: String,
    comic_published_at: NaiveDateTime,
    comic_author: String,
) -> Result<()> {
    bot.send_photo(chat_id, InputFile::url(comic_url))
        .caption(format!(
            "https://explosm.net/comics/{}\n{}\nby {}",
            comic_slug, comic_published_at, comic_author
        ))
        .await
        .context(format!(
            "Failed to send update to: {chat_id}, comic: {comic_slug}"
        ))?;
    Ok(())
}

async fn get_next_js_build_id() -> Result<String> {
    let client = Client::new();
    // Fetch next js build id.
    let resp = client
        .get(EXPLOSM_LATEST_URL)
        .send()
        .await
        .context("Received error response from explosm latest url")?;
    let explosm_html_doc = Html::parse_document(&resp.text().await?);

    // <script id="__NEXT_DATA__" type="application/json">
    // {
    //     ....
    //     "buildId": "-TYFK75-JIdydvgaVJj5B",
    //     ....
    // }
    // </script>
    let next_js_data_element = explosm_html_doc
        .select(
            &Selector::parse("#__NEXT_DATA__")
                .map_err(|e| anyhow::anyhow!("Selector parse error: {e}"))?,
        )
        .next()
        .ok_or_else(|| anyhow::anyhow!("Failed to find next js data in explosm latest url"))?;

    let next_js_data: NextJsData =
        serde_json::from_str(&next_js_data_element.text().collect::<String>())
            .context("Failed to parse JSON from next js data")?;
    Ok(next_js_data.build_id)
}
