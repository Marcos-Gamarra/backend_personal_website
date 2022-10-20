use tokio_postgres::NoTls;

fn get_credentials() -> (String, String) {
    println!("Enter username: ");
    let mut user = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut user).unwrap();
    let password = rpassword::prompt_password("Your password: ").unwrap();

    (user, password)
}

//initialize a pool of connections
pub async fn init_pool() -> bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>> {
    let (user, password) = get_credentials();
    let manager = bb8_postgres::PostgresConnectionManager::new_from_stringlike(
        format!("host=localhost user={} password={}", user, password),
        NoTls,
    )
    .unwrap();

    let pool = bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap();

    pool
}

pub async fn test(
    pool: actix_web::web::Data<bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>>,
) -> String {
    let conn = pool.get().await.unwrap();
    let query = "SELECT word FROM words WHERE word ='clear'";
    let rows = conn.query(query, &[]).await.unwrap();
    let word: String = rows[0].get(0);
    return word;
}

pub async fn check_word(
    word: String,
    pool: actix_web::web::Data<bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>>,
) -> bool {
    let connection = pool.get().await.unwrap();
    let query = "SELECT * from words WHERE word=$1";

    let rows = connection.query(query, &[&word]).await.unwrap();

    if rows.len() == 0 {
        return false;
    }

    true
}

pub async fn get_word(
    pool: actix_web::web::Data<bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>>,
) -> Result<String, std::io::Error> {
    let connection = pool.get().await.unwrap();
    let query = "SELECT * from words ORDER BY random() LIMIT 1";
    let query_result = connection.simple_query(query).await.unwrap();

    if let tokio_postgres::SimpleQueryMessage::Row(row) = &query_result[0] {
        return Ok(row.get(0).unwrap().to_string());
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No rows returned",
        ));
    }
}
