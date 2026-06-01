USE sales;

CREATE TABLE countries (
    country_id INT IDENTITY PRIMARY KEY,
    country_name VARCHAR(100) UNIQUE NOT NULL
) ON [PRIMARY];

CREATE TABLE cities (
    city_id INT IDENTITY PRIMARY KEY,
    city_name VARCHAR(100) NOT NULL,
    country_id INT NOT NULL,
    FOREIGN KEY (country_id) REFERENCES countries(country_id)
) ON [PRIMARY];

CREATE TABLE categories (
    category_id INT IDENTITY PRIMARY KEY,
    category_name VARCHAR(100) UNIQUE NOT NULL
) ON [PRIMARY];

CREATE TABLE statuses (
    status_id INT IDENTITY PRIMARY KEY,
    status_name VARCHAR(50) NOT NULL
) ON [PRIMARY];

CREATE TABLE customers (
    customer_id INT IDENTITY PRIMARY KEY,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    email VARCHAR(150),
    phone VARCHAR(50),
    city_id INT,
    FOREIGN KEY (city_id) REFERENCES cities(city_id)
) ON [current];

CREATE TABLE products (
    product_id INT IDENTITY PRIMARY KEY,
    product_name VARCHAR(100),
    category_id INT,
    price DECIMAL(10,2),
    stock INT,
    FOREIGN KEY (category_id) REFERENCES categories(category_id)
) ON [current];

CREATE TABLE orders (
    order_id INT IDENTITY PRIMARY KEY,
    customer_id INT,
    order_date DATE,
    status_id INT,
    FOREIGN KEY (customer_id) REFERENCES customers(customer_id),
    FOREIGN KEY (status_id) REFERENCES statuses(status_id)
) ON [current];

CREATE TABLE order_details (
    order_id INT,
    product_id INT,
    quantity INT,
    total DECIMAL(10,2),
    PRIMARY KEY (order_id, product_id),
    FOREIGN KEY (order_id) REFERENCES orders(order_id),
    FOREIGN KEY (product_id) REFERENCES products(product_id)
) ON [current];
