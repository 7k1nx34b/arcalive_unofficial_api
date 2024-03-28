static USER_AGENT: &str = "live.arca.android/0.8.369";


pub mod Read {
    use serde_json::Value;
    use crate::{USER_AGENT};

    fn get_headers() -> reqwest::header::HeaderMap {

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("User-Agent", USER_AGENT.parse().unwrap());
        headers.insert("Host", "arca.live".parse().unwrap());
        headers.insert("X-Device-Token", "".parse().unwrap());

        headers
    }

    pub fn read_article(chan_id: &str, article_id: i64) -> reqwest::Result<Value> {
        let http = reqwest::blocking::Client::new();
        let res = http.get(
            format!(
               "https://arca.live/api/app/view/article/{}/{}",
                chan_id, article_id
            )
        )
            .headers(get_headers())
            .send()
            .unwrap();

        return res.json();

    }
    pub fn read_channel(chan_id: &str) -> reqwest::Result<Value> {

        let http = reqwest::blocking::Client::new();
        let res = http.get(
            format!("https://arca.live/api/app/list/channel/{}", chan_id)

        )
            .headers(get_headers())
            .send()
            .unwrap();

        return res.json();


    }

}
