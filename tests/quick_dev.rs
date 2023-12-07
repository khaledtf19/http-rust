use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    hc.do_get("/hello2/kha").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/hello2/kha").await?.print().await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "hello"
        }),
    )
    .await?
    .print()
    .await?;

    // hc.do_delete("/api/tickets/0").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
