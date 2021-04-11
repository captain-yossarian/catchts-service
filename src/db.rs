use actix_web::client::Client;

use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str;

pub struct MysqlClient;
#[derive(Serialize, Deserialize, Debug)]
pub struct IpResponse {
    pub city: String,
    pub country_name: String,
    pub latitude: f32,
    pub longitude: f32,
}

impl Default for IpResponse {
    fn default() -> Self {
        IpResponse {
            city: "unknown".to_string(),
            country_name: "unknown".to_string(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}
const DB_URL:&str = "mysql://ui90ojdqwe2putyy:lPsYs92Zv5qkq6DadOkh@b4wshpjlpwfr1cbfl81o-mysql.services.clever-cloud.com:3306/b4wshpjlpwfr1cbfl81o";

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub customer_id: i32,
    pub amount: i32,
    pub account_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Like {
    pub id: i32,
    pub article_id: i32,
    pub count: i32,
}

impl MysqlClient {
    pub async fn collect(ip: IpAddr) -> Result<()> {
        let client = Client::default();
        let ip_url = format!("https://ipapi.co/{}/json", ip.to_string());
        let response = client
            .get(ip_url)
            .header("User-Agent", "actix-web/3.0")
            .send()
            .await;

        let IpResponse {
            city,
            country_name,
            latitude,
            longitude,
        } = match response.unwrap().json::<IpResponse>().await {
            Ok(db_result) => db_result,
            Err(_) => IpResponse::default(),
        };
        let pool = Pool::new_manual(0, 1, DB_URL)?;
        let mut connection = pool.get_conn().expect("Error get_conn");

        connection.exec_drop(
            r"INSERT INTO Visitors (ip_address, city, country_name, latitude, longitude) VALUES (:ip_address, :city, :country_name, :latitude, :longitude)",
            (ip.to_string(),city,country_name,latitude,longitude),
        )
    }
    pub async fn insert(article_id: i32) -> Result<()> {
        let pool = Pool::new_manual(0, 1, DB_URL)?;

        let mut connection = pool.get_conn().expect("Error get_conn");

        connection.exec_drop(
            r"INSERT INTO Likes (article_id, count) VALUES (:article_id, :count)",
            (article_id, 1),
        )?;

        Ok(())
    }
    pub async fn update(article_id: i32, count: i32) -> Result<()> {
        let pool = Pool::new_manual(0, 1, DB_URL)?;

        let mut connection = pool.get_conn().expect("Error get_conn");

        let query = format!(
            "UPDATE Likes SET count = {} WHERE article_id = {}",
            count, article_id
        );
        connection.query_drop(query)?;

        Ok(())
    }

    pub async fn select(id: String) -> Result<Vec<Like>> {
        let pool = Pool::new_manual(0, 1, DB_URL)?;

        let mut connection = pool.get_conn().expect("Error get_conn");
        let query = format!(
            "SELECT id, article_id, count from Likes WHERE article_id='{}'",
            id
        );

        let likes = connection.query_map(query, |(id, article_id, count)| Like {
            id,
            article_id,
            count,
        })?;

        Ok(likes)
    }
}
