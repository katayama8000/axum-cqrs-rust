# Command Handler設計に関する検討事項

## 現在の実装

### build_command_handler.rs の責任

現在の`build_command_handler`関数は以下の複数の責任を持っています：

1. **CommandHandler依存関係の構築** (本来の責任)
2. **イベントシステムの初期化** 
3. **Redisプロジェクションハンドラーの起動**
4. **バックグラウンドタスクの管理**

```rust
pub fn build_command_handler(
    db: sqlx::MySqlPool,
    redis_client: redis::Client,
) -> CommandHandlerImpl {
    // イベントパブリッシャーの作成
    let (event_publisher, event_receiver) = InMemoryEventPublisher::new();
    let event_publisher: Arc<dyn EventPublisher> = Arc::new(event_publisher);
    
    // Redisハンドラーの作成と起動 (⚠️ 責任の混在)
    let redis_handler = RedisProjectionHandler::new(redis_client.clone(), db.clone());
    tokio::spawn(async move {
        redis_handler.start_processing(event_receiver).await;
    });

    // CommandHandler構築 (本来の責任)
    let circle_repository = Arc::new(CircleRepository::new(db.clone(), event_publisher));
    let circle_duplicate_checker = Arc::new(CircleDuplicateChecker::new(db.clone()));

    CommandHandlerImpl {
        circle_repository,
        circle_duplicate_checker,
    }
}
```

## 設計上の課題

### 1. 単一責任原則違反
- **本来の責任**: CommandHandlerとその依存関係の組み立て
- **現在の追加責任**: Redis処理システムの初期化とバックグラウンドタスク管理

### 2. テスタビリティの低下
- イベントシステムとCommandHandlerが密結合
- 単体テストでCommandHandlerのみをテストすることが困難

### 3. 将来の拡張性への懸念
- 新しいプロジェクションハンドラー追加時に`build_command_handler`を変更する必要
- イベントシステムの変更がCommandHandler構築に影響

## 改善案

### オプション1: アプリケーション初期化レベルで分離

```rust
// app.rs
pub async fn run() -> Result<(), ()> {
    let mysql_pool = mysql_connect().await?;
    let redis_client = redis_connect()?;

    // 1. イベントシステムを先に初期化
    let event_publisher = setup_event_system(redis_client.clone(), mysql_pool.clone()).await;
    
    // 2. Command Handlerは純粋に構築のみ
    let command_handler = build_command_handler(mysql_pool, event_publisher);
    let query_handler = build_query_handler(redis_client);
    
    // 3. アプリケーション起動
    let state = AppState::new(Arc::new(command_handler), Arc::new(query_handler));
    // ...
}

async fn setup_event_system(
    redis_client: redis::Client,
    db: sqlx::MySqlPool,
) -> Arc<dyn EventPublisher> {
    let (event_publisher, event_receiver) = InMemoryEventPublisher::new();
    let redis_handler = RedisProjectionHandler::new(redis_client, db);
    
    tokio::spawn(async move {
        redis_handler.start_processing(event_receiver).await;
    });
    
    Arc::new(event_publisher)
}
```

### オプション2: 専用のEvent System Builder

```rust
pub struct EventSystemBuilder;

impl EventSystemBuilder {
    pub fn build(
        redis_client: redis::Client,
        db: sqlx::MySqlPool,
    ) -> Arc<dyn EventPublisher> {
        let (event_publisher, event_receiver) = InMemoryEventPublisher::new();
        let redis_handler = RedisProjectionHandler::new(redis_client, db);
        
        tokio::spawn(async move {
            redis_handler.start_processing(event_receiver).await;
        });
        
        Arc::new(event_publisher)
    }
}

// build_command_handler.rs
pub fn build_command_handler(
    db: sqlx::MySqlPool,
    event_publisher: Arc<dyn EventPublisher>,
) -> CommandHandlerImpl {
    let circle_repository = Arc::new(CircleRepository::new(db.clone(), event_publisher));
    let circle_duplicate_checker = Arc::new(CircleDuplicateChecker::new(db.clone()));

    CommandHandlerImpl {
        circle_repository,
        circle_duplicate_checker,
    }
}
```

### オプション3: Dependency Injection Container

```rust
pub struct AppContainer {
    event_publisher: Arc<dyn EventPublisher>,
    mysql_pool: sqlx::MySqlPool,
    redis_client: redis::Client,
}

impl AppContainer {
    pub async fn new(mysql_pool: sqlx::MySqlPool, redis_client: redis::Client) -> Self {
        let event_publisher = Self::setup_event_system(redis_client.clone(), mysql_pool.clone()).await;
        
        Self {
            event_publisher,
            mysql_pool,
            redis_client,
        }
    }
    
    pub fn build_command_handler(&self) -> CommandHandlerImpl {
        // 純粋な依存関係注入
        build_command_handler(self.mysql_pool.clone(), self.event_publisher.clone())
    }
    
    async fn setup_event_system(
        redis_client: redis::Client,
        db: sqlx::MySqlPool,
    ) -> Arc<dyn EventPublisher> {
        // イベントシステム初期化
    }
}
```

## 推奨アクション

### 短期的 (現状維持)
- 現在の実装は動作しているため、急いで変更する必要はない
- この設計課題をドキュメント化して、将来のリファクタリングの指針とする

### 中期的 (リファクタリング)
- **オプション1**を推奨: `app.rs`レベルでの責任分離
- テストしやすさとコードの可読性が向上
- 段階的にリファクタリング可能

### 長期的 (アーキテクチャ改善)
- Dependency Injection Containerの導入検討
- より複雑なイベントシステムへの対応
- マイクロサービス化への準備

## 関連ファイル

- `/workspace/src/crates/main/src/injectors/build_command_handler.rs`
- `/workspace/src/crates/infrastructure/src/event_publisher.rs`
- `/workspace/src/crates/main/src/app.rs`
- `/workspace/Description/EVENT_DRIVEN_REDIS.md`

## メモ

この設計課題は2025年10月9日時点で特定されました。現在のシステムは正常に動作していますが、将来の保守性と拡張性のために改善を検討する価値があります。