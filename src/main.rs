use reqwest;
use serde_json::value::Value;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ResponseData {
    bots: Vec<Value>,
    _total: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Chatter {
    login: String,
    __typename: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChattersInfo {
    broadcasters: Vec<Chatter>,
    staff: Vec<Chatter>,
    moderators: Vec<Chatter>,
    vips: Vec<Chatter>,
    viewers: Vec<Chatter>,
    count: i32,
    __typename: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Channel {
    id: String,
    chatters: ChattersInfo,
    __typename: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatViewersResponseItemData {
    channel: Channel,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatViewersResponseItemExtensions {
    #[serde(rename = "durationMilliseconds", alias = "durationMilliseconds")]
    duration_milliseconds: i32,
    #[serde(rename = "operationName", alias = "operationName")]
    operation_name: String,
    #[serde(rename = "requestID", alias = "requestID")]
    request_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatViewersResponseItem {
    data: ChatViewersResponseItemData,
    extensions: ChatViewersResponseItemExtensions,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatViewersRequestDataVariables {
    #[serde(rename = "channelLogin", alias = "channelLogin")]
    channel_login: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PersistedQuery {
    version: i32,
    #[serde(rename = "sha256Hash", alias = "sha256Hash")]
    sha256_hash: String,

}
#[derive(Serialize, Deserialize, Debug)]
struct DataExtensions {
    #[serde(rename = "persistedQuery", alias = "persistedQuery")]
    persisted_query: PersistedQuery
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatViewersRequestData {
    #[serde(rename = "operationName", alias = "operationName")]
    operation_name: String,
    variables: ChatViewersRequestDataVariables,
    extensions: DataExtensions,
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("usage: twitch-bots CHANNEL_NAME");
        return;
    }

    let twitch_channel = String::from(&args[1]);
    let client = reqwest::Client::new();

    // ---------------------------------------------------------------
    // Get all bots by checking twitchinsights
    // ---------------------------------------------------------------
    let resp_data = client
        .get("https://api.twitchinsights.net/v1/bots/all")
        .send()
        .await
        .unwrap()
        .json::<ResponseData>()
        .await
        .unwrap();
    let mut bot_names: Vec<String> = Vec::new();
    for bot in resp_data.bots {
        bot_names.push(String::from(bot.as_array().unwrap()[0].as_str().unwrap()));
    }
    // println!("{:?}", bot_names);


    // ---------------------------------------------------------------
    // Get all currently connected users in chat using twitch gql
    // ---------------------------------------------------------------

    let request_data: Vec<ChatViewersRequestData> = vec![
        ChatViewersRequestData {
            operation_name: String::from("ChatViewers"),
            variables: ChatViewersRequestDataVariables {
                channel_login: String::from(&twitch_channel),
            },
            extensions: DataExtensions {
                persisted_query: PersistedQuery {
                    version: 1,
                    sha256_hash: String::from("e0761ef5444ee3acccee5cfc5b834cbfd7dc220133aa5fbefe1b66120f506250"),
                },
            },
        }
    ];

    let data: String = serde_json::to_string(&request_data).unwrap();
    let resp_data = client
        .post("https://gql.twitch.tv/gql#origin=twilight")
        .header("Client-Id", "kimne78kx3ncx6brgo4mv6wki5h1ko")
        .body(data)
        .send()
        .await
        .unwrap()
        .json::<Vec<ChatViewersResponseItem>>()
        .await
        .unwrap();

    // println!("{:?}", &resp_data[0]);
    let mut current_users: Vec<String> = Vec::new();
    for x in &resp_data[0].data.channel.chatters.broadcasters {
        current_users.push(String::from(&x.login));
    }
    for x in &resp_data[0].data.channel.chatters.moderators {
        current_users.push(String::from(&x.login));
    }
    for x in &resp_data[0].data.channel.chatters.staff {
        current_users.push(String::from(&x.login));
    }
    for x in &resp_data[0].data.channel.chatters.vips {
        current_users.push(String::from(&x.login));
    }
    for x in &resp_data[0].data.channel.chatters.viewers {
        current_users.push(String::from(&x.login));
    }


    // ---------------------------------------------------------------
    // Print bot users that are also current chat users
    // ---------------------------------------------------------------

    // println!("{:?}", current_users);
    let bot_users: Vec<String> = current_users.into_iter().filter(|x| bot_names.contains(x)).collect();
    println!("{}", bot_users.join("\n"));
}
