// use envs::envs::get_mongo_uri;
use general::envs::get_env;
use mongodb::{error::Error, Database};

fn get_mongo_url() -> String {
    get_env("MONGO_URI").unwrap()
}

pub async fn connect(database: &str) -> Result<Database, Error> {
    tracing::info!("Attempting to connect to MongoDB");

    // let s = get_env("MONGO_URI")?;
    // let s= get_env("MONGO_URI").map_err(|e| Error::from(e))?;
    let client = mongodb::Client::with_uri_str(get_mongo_url())
        .await
        .inspect(|_| tracing::info!("Successfully connected to MongoDB"))
        .inspect_err(|e| {
            tracing::error!("Failed to connect to MongoDB, error: {}", e.to_string());
            std::process::exit(1);
        })?;
    // client
    //     .database(database)
    //     .run_command(mongodb::bson::doc! {"ping": 1})
    //     .await?;
    Ok(client.database(database))
}

// fn get_env_or_panic(key: &str) -> String {
//     std::env::var(key).expect(&format!("Environment variable {key} must be set"))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() -> Result<(), Error> {
        let db_name = "test_db";
        let db = connect(db_name).await?;

        assert_eq!(db.name(), db_name);
        Ok(())
    }
}
