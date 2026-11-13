use learning_collection::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_learnings() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/learning").await;
        assert_eq!(res.status_code(), 200);
        res.assert_json(&serde_json::json!([]));
    })
    .await;
}
