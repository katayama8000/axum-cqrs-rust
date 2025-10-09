# 📦 Circle 集約 - CQRS + イベントソーシング テーブル設計

このドキュメントは、`Circle` 集約における **Command（書き込みモデル）** のテーブル設計と、**Query（読み取りモデル）** のアーキテクチャについて説明します。  
読み取りモデルはRedisに格納されます。  
アーキテクチャとしては CQRS + Event Sourcing を採用しています。

---

## 📝 Command: `circle_events`

Circle 集約に対するすべてのドメインイベントを保存する **イベントストアテーブル** です。  
このテーブルを唯一のソース・オブ・トゥルース（Single Source of Truth）とし、状態の再構築はすべてイベントリプレイにより行います。

```sql
CREATE TABLE circle_events (
    id CHAR(36) PRIMARY KEY,                -- イベントID（UUID）
    circle_id CHAR(36) NOT NULL,            -- 集約ID（Circle ID）
    version INT NOT NULL,                   -- バージョン（楽観ロックに使用）
    event_type VARCHAR(100) NOT NULL,       -- イベント名（例: CircleCreated）
    payload JSON NOT NULL,                  -- イベント内容（差分 or 全体のスナップショット）
    occurred_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, -- イベント発生日時
);
```

## 📦 イベント例（payload の中身）

以下は、circle_events.payload に格納される JSON の具体例です。
Rust 側でのイベント enum や struct にマッピングできます。

```json
{
    "name": "サークル名",
    "capacity": 50,
}
```
```json
{
    "name": "新しいサークル名"
}
```
```json
{
    "capacity": 100
}
```

## 💾 スナップショット

イベントストリームが長くなると、集約の再構築に時間がかかるようになります。そこでスナップショットを活用します。
スナップショットは特定バージョンにおける集約の完全な状態を保存し、リハイドレーション（再構築）の効率化に役立ちます。

```sql
CREATE TABLE circle_snapshots (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,    -- スナップショットの一意識別子
    circle_id CHAR(36) NOT NULL,             -- 集約ID（Circle ID）
    version INT NOT NULL,                    -- スナップショット時点のバージョン
    state JSON NOT NULL,                     -- 集約の完全な状態
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, -- スナップショット作成日時
    INDEX idx_circle_version (circle_id, version DESC)      -- 検索の効率化用インデックス
);
```

### スナップショットの例（state の中身）

```json
{
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "プログラミング部",
    "capacity": 100,
    "created_at": "2023-01-15T09:30:00Z",
    "version": 5
}
```

スナップショットは定期的に（例：バージョンが一定数増えるごと）または必要に応じて作成します。
集約の読み込み時は、最新のスナップショットから状態を復元し、そこから最新までのイベントのみを適用することで、
処理効率を大幅に向上させることができます。