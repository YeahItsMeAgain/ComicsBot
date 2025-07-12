use anyhow::Result;
use chrono::NaiveDateTime;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{cmp, collections::HashMap};

use crate::store::get_store;

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
pub async fn notify_changes() -> Result<()> {
    let next_js_build_id = get_next_js_build_id().await?;
    log::info!("Received next js build id: {next_js_build_id}");

    let comics_data =
        reqwest::get(EXPLOSM_NEXT_JS_COMICS_URL.replace("{next_js_build_id}", &next_js_build_id))
            .await?
            .json::<ComicsResponseData>()
            .await?;

    let mut max_comic_timestamp = get_store()
        .get(EXPLOSM_LAST_COMIC_TIMESTAMP_KEY)
        .unwrap_or("0".to_owned())
        .parse::<i64>()
        .unwrap();

    for (_year, comics) in comics_data.props.comics_archive {
        for (_index, comics) in comics {
            for comic in comics {
                let Some(published_at) = comic.publish_at else {
                    continue;
                };
                let published_at =
                    NaiveDateTime::parse_from_str(&published_at, "%Y-%m-%d %H:%M:%S")?;
                let published_at_timestamp = published_at.and_utc().timestamp();
                max_comic_timestamp = cmp::max(published_at_timestamp, max_comic_timestamp);

                println!("  Comic: {:#?}", comic.slug);
            }
        }
    }

    get_store()
        .set(
            EXPLOSM_LAST_COMIC_TIMESTAMP_KEY,
            max_comic_timestamp.to_string(),
        )
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to save last explosm comic timestamp: {max_comic_timestamp}, {e}"
            )
        })?;
    Ok(())
}

async fn get_next_js_build_id() -> Result<String> {
    let client = Client::new();
    // Fetch next js build id.
    let resp = client.get(EXPLOSM_LATEST_URL).send().await.map_err(|err| {
        anyhow::anyhow!("Received error response from explosm latest url, {}", err)
    })?;
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
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON from next js data: {e}"))?;
    Ok(next_js_data.build_id)
}
