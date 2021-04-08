use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use std::str;

pub struct Client;

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

impl Client {
    pub async fn collect(ip: IpAddr) -> Result<()> {
        let pool = Pool::new_manual(0, 1, DB_URL)?;
        let mut connection = pool.get_conn().expect("Error get_conn");

        connection.exec_drop(
            r"INSERT INTO visitors (ip_address) VALUES (:ip_address)",
            (ip.to_string(),),
        )?;

        Ok(())
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
