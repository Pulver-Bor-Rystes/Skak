// use mysql::Pool;

// use crate::types::Username;

// fn insert_cookie(username: Username, cookie: String, pool: &Pool) -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let mut conn = pool.get_conn()?;

//     // Now let's insert payments to the database
//     conn.exec_batch(
//         r"INSERT INTO payment (customer_id, amount, account_name)
//           VALUES (:customer_id, :amount, :account_name)",
//         payments.iter().map(|p| params! {
//             "customer_id" => p.customer_id,
//             "amount" => p.amount,
//             "account_name" => &p.account_name,
//         })
//     )?;

//     Ok(())
// }
