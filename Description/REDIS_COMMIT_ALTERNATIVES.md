# MySQLトリガーでRedisコミットに関する代替案

## 問題点
現在のstoreメソッドは2段階コミット問題を抱えています：
1. MySQLへのイベント保存
2. Redisキャッシュの更新

## 解決策

### 1. イベント駆動アーキテクチャ（推奨）
- MySQLにイベントを保存後、非同期でRedisを更新
- イベントパブリッシャー/サブスクライバーパターンを使用
- 最終的整合性を受け入れる

### 2. Redis を Query-only にする
- Redisは読み取り専用キャッシュとして扱う
- キャッシュミスの場合はMySQLから再構築
- Redis更新失敗はログ出力のみで継続

### 3. Outbox Pattern
- MySQLにイベントテーブルとoutboxテーブルを作成
- 同一トランザクション内でoutboxにRedis更新命令を保存
- 別プロセスでoutboxを監視してRedis更新

### 4. Change Data Capture (CDC)
- Debezium等を使ってMySQLの変更を監視
- 変更を検出したらRedisを更新
- プロジェクトにKafka/Debeziumの設定が既にある

### 5. Saga Pattern
- 分散トランザクションとして扱う
- MySQLコミット後にRedis更新
- Redis更新失敗時の補償トランザクション

## MySQLトリガーが困難な理由
1. 外部システム（Redis）への接続サポートなし
2. ネットワーク操作の制限
3. トランザクション境界の問題
4. エラーハンドリングの困難さ

## 推奨アプローチ
イベント駆動アーキテクチャ + 最終的整合性
- 信頼性: MySQLがSingle Source of Truth
- パフォーマンス: Redis更新は非同期
- 復旧性: Redis障害時もMySQLから復旧可能

## 実際に採用したアプローチ

**イベント駆動アーキテクチャ（案1）** を実装しました。

### 実装内容
1. **EventPublisher trait**: イベントの非同期発行を抽象化
2. **InMemoryEventPublisher**: tokio::mpscを使った軽量実装
3. **RedisProjectionHandler**: イベント受信→Redis更新処理
4. **CircleRepository改修**: イベントパブリッシャー統合

### 動作フロー
```
1. Circle作成/更新リクエスト
   ↓
2. MySQLにイベント保存（トランザクション）
   ↓
3. EventPublisher経由でイベント発行
   ↓
4. RedisProjectionHandler（バックグラウンド）
   ↓ 
5. MySQLからCircle状態を再構築
   ↓
6. Redisにキャッシュ保存
```

### 利点
- **整合性**: MySQLコミット成功が保証された後にRedis更新
- **パフォーマンス**: Redis処理がメインフローをブロックしない
- **障害耐性**: Redis障害時もサービス継続可能
- **拡張性**: 複数のプロジェクションハンドラー追加可能

### トレードオフ
- **最終的整合性**: 短期間のRedis-MySQL間不整合
- **複雑性**: シンプルな同期更新と比べて複雑
- **デバッグ**: 非同期処理のトレースが困難