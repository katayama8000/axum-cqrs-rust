# 📦 Circle 集約 - CQRS + イベントソーシング テーブル設計

このドキュメントは、`Circle` 集約における **Command（書き込みモデル）** および **Query（読み取りモデル）** のテーブル設計を説明します。  
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

## 🔍 Query: circle_projections

こちらは読み取り用に最適化されたプロジェクションテーブルです。
コマンド側のイベントから状態を構築・更新し、クエリの高速化や API 応答に利用します。

```sql
CREATE TABLE circle_projections (
    circle_id CHAR(36) PRIMARY KEY,         -- 集約ID（Circle ID）
    name VARCHAR(100) NOT NULL,             -- サークル名
    capacity SMALLINT NOT NULL,             -- 定員
    version INT NOT NULL,                   -- 最新バージョン
    last_event_at DATETIME NOT NULL         -- 最終イベント発生日時
);
```

## 📦 イベント例（payload の中身）

以下は、circle_events.payload に格納される JSON の具体例です。
Rust 側でのイベント enum や struct にマッピングできます。

```json
{
    "circle_id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "サークル名",
    "capacity": 50,
    "created_at": "2023-10-01T12:00:00Z"
}
```
```json
{
    "circle_id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "新しいサークル名"
}
```
```json
{
    "circle_id": "123e4567-e89b-12d3-a456-426614174000",
    "capacity": 100
}
```