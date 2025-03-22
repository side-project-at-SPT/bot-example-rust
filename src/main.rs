use axum::{
    routing::{get, post},
    Router,
    Json,
};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

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
    
    // 創建路由
    let app = Router::new()
        .route("/", get(|| async { "Bot Player Service" }))
        .route("/make-move", post(make_move));
    
    // 設定服務地址
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("服務啟動於 http://localhost:3000");
    
    // 啟動服務
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
