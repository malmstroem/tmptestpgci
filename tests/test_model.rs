#[tokio::test]
async fn test_model() {
    let pool = tmptestpgci::connect().await.unwrap();
    tmptestpgci::init_db(&pool).await.unwrap();
    tmptestpgci::populate_db(&pool).await.unwrap();
    tmptestpgci::query_db(&pool).await.unwrap();
}
