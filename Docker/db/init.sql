-- データベースの作成
CREATE DATABASE IF NOT EXISTS mydatabase;

-- myuserに全ての権限を付与
GRANT ALL PRIVILEGES ON mydatabase.* TO 'myuser' @'%' IDENTIFIED BY 'mypassword';

-- mydatabaseを使用する
USE mydatabase;

-- Circlesテーブルの作成 (owner_id カラム削除)
CREATE TABLE IF NOT EXISTS circles (
    id CHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    capacity INT NOT NULL
);

-- Circlesテーブルへの初期データ挿入
INSERT INTO
    circles (id, name, capacity)
VALUES
    (UUID(), 'Circle A', 5),
    (UUID(), 'Circle B', 8),
    (UUID(), 'Circle C', 10);