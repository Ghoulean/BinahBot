use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::ArgAction;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

use crate::ddb::put_deck;
use crate::models::deck::Deck;
use crate::models::deck::TiphDeck;
use crate::thumbnail::generate_thumbnail;
use crate::tiph::decode;

// Taken from https://rust-cli-recommendations.sunshowers.io/handling-arguments.html
#[derive(Debug, Parser)]
#[clap(name = "bb-ops", version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    DeckQuickfill {
        #[clap(long, short = 'u')]
        username: String,
        #[clap(long, short = 'i')]
        user_id: String,
        csv_path: PathBuf,
        ddb_table_name: String,
        thumbnail_function_name: String,
    },
}

#[derive(Debug, Args)]
struct GlobalOpts {
    #[clap(long, short, global = true, action = ArgAction::Count)]
    verbose: u8,
}

impl App {
    pub async fn exec(
        self,
        ddb_client: &aws_sdk_dynamodb::Client,
        lambda_client: &aws_sdk_lambda::Client,
        http_client: &reqwest::Client
    ) -> Result<(), ()> {
        match self.command {
            Command::DeckQuickfill { username, user_id, csv_path, ddb_table_name, thumbnail_function_name } => {
                let f = File::open(csv_path).expect("can't read csv");
                let mut csv_reader = csv::Reader::from_reader(BufReader::new(f));
                for result in csv_reader.records() {
                    let record = result.expect("can't read csv line");
                    let deck_name = record.get(0).expect("can't get deck name from csv");
                    let deck_code = record.get(1).expect("can't get deck code from csv");
                    let description = record.get(2);

                    let tiph_deck = TiphDeck(deck_code.to_string(), 1);
                    let deck_data = decode(http_client, &tiph_deck).await.expect("couldn't get tiph deck data");

                    let thumbnail_result = generate_thumbnail(
                        lambda_client,
                        &thumbnail_function_name,
                        &deck_data.combat_page_ids,
                    ).await;
                
                    let deck = Deck {
                        name: deck_name.to_string(),
                        author_id: user_id.to_string(),
                        author_name: username.to_string(),
                        description: description.map(|x| x.to_string()),
                        deck_data,
                        tiph_deck: Some(tiph_deck),
                    };

                    let put_deck_result = put_deck(
                        ddb_client,
                        &ddb_table_name,
                        &deck,
                        true,
                    )
                    .await;

                    match thumbnail_result {
                        Ok(_) => println!("Successfully thumbnailed for \"{}\"", deck_name),
                        Err(_) => println!("FAILED THUMBNAIL: {:?}", record),
                    };
                    match put_deck_result {
                        Ok(_) => println!("Successfully put \"{}\"", deck_name),
                        Err(_) => println!("FAILED DDB PUT: {:?}", record),
                    };
                }
                Ok(())
            }
        }
    }
}
