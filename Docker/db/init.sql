-- データベースの作成
CREATE DATABASE IF NOT EXISTS mydatabase;

-- myuserに全ての権限を付与
GRANT ALL PRIVILEGES ON mydatabase.* TO 'myuser' @'%' IDENTIFIED BY 'mypassword';

-- 使用するデータベースを指定
USE mydatabase;

-- イベントストア: circle_events テーブルの作成
CREATE TABLE IF NOT EXISTS circle_events (
    id CHAR(36) NOT NULL PRIMARY KEY,
    circle_id CHAR(36) NOT NULL,
    version INT NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    payload JSON NOT NULL,
    occurred_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_circle_version (circle_id, version)
);

-- プロジェクション: circle_projections テーブルの作成
CREATE TABLE IF NOT EXISTS circle_projections (
    circle_id CHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    capacity INT NOT NULL,
    version INT NOT NULL,
    last_event_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Circle スナップショットテーブル
CREATE TABLE IF NOT EXISTS circle_snapshots (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    circle_id CHAR(36) NOT NULL,
    version INT NOT NULL,
    state JSON NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_circle_version (circle_id, version DESC)
);