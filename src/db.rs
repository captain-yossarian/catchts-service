use mysql_async::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

use std::str;

pub struct Client;

const database_url:&str = "mysql://uovdcifhncev7fxi:Wjm2xWWw884lOhjYl7jL@bwxhllqvglwprogbjyd4-mysql.services.clever-cloud.com:3306/bwxhllqvglwprogbjyd4";

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub customer_id: i32,
    pub amount: i32,
    pub account_name: Option<String>,
}

impl Client {
    pub async fn new() -> std::result::Result<Vec<Payment>, mysql_async::Error> {
        let payments = vec![
            Payment {
                customer_id: 1,
                amount: 2,
                account_name: None,
            },
            Payment {
                customer_id: 3,
                amount: 4,
                account_name: Some("foo".into()),
            },
            Payment {
                customer_id: 5,
                amount: 6,
                account_name: None,
            },
            Payment {
                customer_id: 7,
                amount: 8,
                account_name: None,
            },
            Payment {
                customer_id: 9,
                amount: 10,
                account_name: Some("bar".into()),
            },
        ];

        let pool = mysql_async::Pool::new(database_url);
        let mut conn = pool.get_conn().await?;

        // // Create temporary table
        // conn.query_drop(
        //     r"CREATE TABLE payment (
        //         customer_id int not null,
        //         amount int not null,
        //         account_name text
        //     )",
        // )
        // .await?;

        // // Save payments
        // let params = payments.clone().into_iter().map(|payment| {
        //     params! {
        //         "customer_id" => payment.customer_id,
        //         "amount" => payment.amount,
        //         "account_name" => payment.account_name,
        //     }
        // });

        // conn.exec_batch(
        //     r"INSERT INTO payment (customer_id, amount, account_name)
        //       VALUES (:customer_id, :amount, :account_name)",
        //     params,
        // )
        // .await?;

        // Load payments from database. Type inference will work here.

        let loaded_payments = conn
            .exec_map(
                "SELECT customer_id, amount, account_name FROM payment",
                (),
                |(customer_id, amount, account_name)| Payment {
                    customer_id,
                    amount,
                    account_name,
                },
            )
            .await?;

        // Dropped connection will go to the pool
        conn;

        // Pool must be disconnected explicitly because
        // it's an asynchronous operation.
        pool.disconnect().await;
        println!("payments {:?}", loaded_payments);

        //  assert_eq!(loaded_payments, payments);

        // the async fn returns Result, so
        Ok(loaded_payments)
    }
}
