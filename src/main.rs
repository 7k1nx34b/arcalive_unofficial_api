use arcalive_unofficial_api::Read;
fn main() {

    let channel = Read::read_channel("vrchat").unwrap();
    for article_info in channel.get("articles").unwrap().as_array().unwrap() {

        let article_id = article_info.get("id")
            .unwrap()
            .as_i64()
            .unwrap();

        let article = Read::read_article("vrchat", article_id)
            .unwrap();

        println!("{:#?}", article);


    }

}