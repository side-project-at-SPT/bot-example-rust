use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

// 這裡的結構將根據你提供的 JSON 格式調整
#[derive(Deserialize)]
struct GameState {
    // 遊戲狀態欄位，可以先放一個示例字段
    game_id: String,
}

#[derive(Serialize)]
struct BotMove {
    // Bot 回應的動作，可以先放一個示例字段
    action: String,
}

async fn make_move(Json(game_state): Json<GameState>) -> Json<BotMove> {
    // 簡單的回應，實際邏輯待實現
    println!("收到遊戲 ID: {}", game_state.game_id);

    Json(BotMove {
        action: "move_forward".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // 初始化日誌
    tracing_subscriber::fmt::init();

    // 從環境變數獲取端口，如果沒有設置則使用默認值 3000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT 必須是有效的數字");

    // 創建路由
    let app = Router::new()
        .route("/", get(|| async { "Bot Player Service" }))
        .route("/make-move", post(make_move));

    // 設定服務地址，使用環境變數中的端口
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("服務啟動於 http://localhost:{}", port);

    // 啟動服務
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
