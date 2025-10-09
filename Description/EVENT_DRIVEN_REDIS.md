# イベント駆動アーキテクチャによるRedis永続化

## 実装概要

2段階コミット問題を解決するため、以下のイベント駆動アーキテクチャを実装しました：

### 1. EventPublisher トレイト
- イベントの非同期発行を抽象化
- MySQLコミット後にイベントを発行

### 2. InMemoryEventPublisher
- インメモリチャネルを使用した軽量実装
- 開発・テスト環境に適している

### 3. RedisProjectionHandler
- イベントを受信してRedisを更新
- MySQLから完全なCircle状態を再構築
- 障害時の自動復旧機能

### 4. 修正されたCircleRepository
- イベントパブリッシャーをオプションでサポート
- 後方互換性を維持
- エラー処理の改善

## アーキテクチャの利点

### 1. 整合性の保証
- MySQLが Single Source of Truth
- Redis更新失敗がメイン処理に影響しない

### 2. パフォーマンス
- Redis更新は非同期実行
- メインフローをブロックしない

### 3. 復旧性
- Redis障害時もMySQLから復旧可能
- イベント再生による状態復元

### 4. スケーラビリティ
- 複数のRedis投影ハンドラーを実行可能
- 水平スケーリングに対応

## 使用方法

```rust
// build_command_handler.rs
let (event_publisher, event_receiver) = InMemoryEventPublisher::new();
let event_publisher: Arc<dyn EventPublisher> = Arc::new(event_publisher);

let redis_handler = RedisProjectionHandler::new(redis_client.clone());
tokio::spawn(async move {
    redis_handler.start_processing(event_receiver).await;
});

let circle_repository = Arc::new(CircleRepository::with_event_publisher(
    db.clone(), 
    redis_client,
    event_publisher
));
```

## 今後の改善点

1. **Kafka/RabbitMQ統合**: より堅牢なメッセージングシステム
2. **リトライ機能**: Redis更新失敗時の自動リトライ
3. **デッドレターキュー**: 処理不可能なイベントの管理
4. **監視・アラート**: Redis同期状態の監視

## 注意事項

- 最終的整合性モデルを採用
- 短期間のRedis-MySQL間の不整合が発生する可能性
- 重要な読み取り処理ではMySQLからの再取得を検討