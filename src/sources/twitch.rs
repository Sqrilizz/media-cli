pub fn is_twitch_url(query: &str) -> bool {
    query.contains("twitch.tv/")
}

pub fn extract_channel(url: &str) -> String {
    if let Some(channel) = url.split("twitch.tv/").nth(1) {
        channel.split('/').next().unwrap_or(url).to_string()
    } else {
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_channel_from_url() {
        assert_eq!(
            extract_channel("https://www.twitch.tv/sqrilizz/videos"),
            "sqrilizz"
        );
    }

    #[test]
    fn preserves_plain_channel() {
        assert_eq!(extract_channel("sqrilizz"), "sqrilizz");
    }
}
