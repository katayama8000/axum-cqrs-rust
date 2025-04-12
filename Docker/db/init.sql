-- データベースの作成
CREATE DATABASE IF NOT EXISTS mydatabase;

-- myuserに全ての権限を付与
GRANT ALL PRIVILEGES ON mydatabase.* TO 'myuser' @'%' IDENTIFIED BY 'mypassword';

-- 使用するデータベースを指定
USE mydatabase;

-- イベントストア: circle_events テーブルの作成
CREATE TABLE IF NOT EXISTS circle_events (
    id CHAR(36) NOT NULL PRIMARY KEY,
    -- イベントID（UUID）
    circle_id CHAR(36) NOT NULL,
    -- 集約ID（Circle ID）
    version INT NOT NULL,
    -- バージョン番号（Aggregateごとに増加）
    event_type VARCHAR(100) NOT NULL,
    -- イベント名（例: CircleCreated）
    payload JSON NOT NULL,
    -- イベント内容（差分 or 全体）
    occurred_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP -- イベント発生日時
);

-- プロジェクション: circle_projections テーブルの作成
CREATE TABLE IF NOT EXISTS circle_projections (
    circle_id CHAR(36) NOT NULL PRIMARY KEY,
    -- 集約ID（Circle ID）
    name VARCHAR(255) NOT NULL,
    -- サークル名
    capacity INT NOT NULL,
    -- 定員
    version INT NOT NULL,
    -- 最新バージョン
    last_event_at DATETIME NOT NULL -- 最終イベント発生日時
);

-- プロジェクションへの初期データ挿入（例: CircleCreated イベント後に適用された状態）
INSERT INTO
    circle_projections (
        circle_id,
        name,
        capacity,
        version,
    )
VALUES
    (UUID(), 'Circle A', 5, 1),
    (UUID(), 'Circle B', 8, 1),
    (UUID(), 'Circle C', 10, 1);