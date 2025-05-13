-- Your SQL goes here
CREATE TABLE products (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    details TEXT NOT NULL  -- 存储 JSON 数据
);
INSERT INTO products (name, details)
VALUES (
    'Smartphone',
    '{"brand": "Apple", "model": "iPhone 13", "specs": {"storage": "128GB", "color": "Midnight"}, "price": 799.99}'
);

INSERT INTO products (name, details)
VALUES (
    'Laptop',
    '{"brand": "Dell", "model": "XPS 15", "specs": {"ram": "16GB", "storage": "512GB"}, "price": 1499.99}'
);
