use actix_web::client::Client;

use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub pathname: String,
    pub session: i32,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct TableRow {
    id: String,
    ip_address: String,
    created_at: String,
    city: Option<String>,
    country_name: Option<String>,
    latitude: Option<f32>,
    longitude: Option<f32>,
}

type TableRowRespone = (
    String,
    String,
    String,
    Option<String>,
    Option<String>,
    Option<f32>,
    Option<f32>,
);
fn map_ip_address(
    (id, ip_address, created_at, city, country_name, latitude, longitude): TableRowRespone,
) -> TableRow {
    TableRow {
        id,
        ip_address,
        created_at,
        city,
        country_name,
        latitude,
        longitude,
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DistinctPerDay {
    date: String,
    distinct_ips: i32,
}

impl MysqlClient {
    pub async fn collect(data: Data) -> Result<()> {
        let Data {
            session, pathname, ..
        } = data;
        let pool = Pool::new_manual(0, 1, DB_URL)?;
        let mut connection = pool.get_conn().expect("Unable to connect to database");

        connection.exec_drop(
            r"INSERT INTO session (visitor_id, pathname) VALUES (:visitor_id, :pathname)",
            (session.to_string(), pathname),
        )
    }
    pub async fn session(ip: IpAddr) -> Result<u64> {
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

        let insert  = connection.exec_drop(
            r"INSERT INTO visitors (ip_address, city, country_name, latitude, longitude) VALUES (:ip_address, :city, :country_name, :latitude, :longitude)",
            (ip.to_string(),city,country_name,latitude,longitude),
        );
        match insert {
            Ok(_) => Ok(connection.last_insert_id()),
            Err(err) => Err(err),
        }
    }
    pub async fn insert(article_id: i32) -> Result<()> {
        let pool = Pool::new_manual(0, 1, DB_URL)?;

        let mut connection = pool.get_conn().expect("Error get_conn");

        connection.exec_drop(
            r"INSERT INTO likes (article_id, count) VALUES (:article_id, :count)",
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

    pub async fn metrics(select_by: String) -> Result<String> {
        let data = match &select_by[..] {
            "distinct_per_day" => {
                let query = r"SELECT  DATE(created_at) Date, COUNT(DISTINCT ip_address) distinct_ips
                            FROM    visitors
                            GROUP   BY  DATE(created_at)";
                let pool = Pool::new_manual(0, 1, DB_URL)?;
                let mut connection = pool.get_conn().expect("Error get_conn");
                let result = connection.query_map(query, |(date, distinct_ips): (String, i32)| {
                    DistinctPerDay { date, distinct_ips }
                });

                match result {
                    Ok(val) => match serde_json::to_string(&val) {
                        Ok(v) => v,
                        Err(_) => String::from("{}"),
                    },
                    Err(_) => String::from("{}"),
                }
            }
            _ => String::from("{}"),
        };
        Ok(data)
        //https://ipapi.co/api/?shell#introduction
    }
}
