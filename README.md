1. 创建工程
    ```
    cargo new rust_diesel_mysql_demo
    ```
1. 添加 diesel 依赖
   diesel = { version = "2.0.0", features = ["mysql"] }
2. mysql 连接信息写入到 env 文件
    ```
    echo DATABASE_URL=mysql://root:123456@localhost:3306/diesel_demo >.env
    ```

4. `cargo install diesel_cli --no-default-features --features mysql`
5. `diesel setup`
   1. 生成 migrations 存放初始化 sql 文件的目录。
   2. 生成 diesel.toml，存放生成文件的目录
       ```
       [print_schema]
       file = "src/schema.rs"
       
       [migrations_directory]
       dir = "migrations"
       ```
3. diesel migration generate create_posts
   生成初始化 sql 文件。
    ```
    Creating migrations/2022-11-22-085601_create_posts/up.sql
    Creating migrations/2022-11-22-085601_create_posts/down.sql
    ```
   将初始化 sql 写到 up.sql 中。
    ```sql
    CREATE TABLE `posts` (
      `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'ID',
      `title` varchar(256) NOT NULL COMMENT '标题',
      `body` text NOT NULL COMMENT '内容',
      `published` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否发布',
      PRIMARY KEY (`id`)
    ) ;
    ```
1. diesel migration run
   1. 读取上一步生成的 migrations 文件夹的 up.sql，生成对应的实体，生成到 print_schema 指定的文件
   2. 如果上一步没有执行，migrations 里是空的，默认会生成数据库里所有表的实体

---

以上就是使用 diesel 的准备工作，接下来就是编码工作

1. 在 src 目录下创建文件夹 api、database、models、service
    ```
    ├── src
    │   ├── api
    │   │   ├── mod.rs
    │   │   └── test_api.rs
    │   ├── database
    │   │   ├── mod.rs
    │   │   ├── mysql.rs
    │   │   └── schema.rs
    │   ├── main.rs
    │   ├── models
    │   │   ├── mod.rs
    │   │   └── models.rs
    │   ├── service
    │   │   ├── mod.rs
    │   │   └── test_service.rs
    │   └── settings.rs
    ```

   1. rust 的目录结构还在摸索中，这里还是 mvc 的方式，
   2. api 提供服务的接口，类似于 controller
   3. database 数据库相关，初始化数据库连接以及 schema
   4. models 代码运行过程中的 struct
   5. service 服务逻辑处理

   

---

https://diesel.rs/guides/getting-started
https://www.rectcircle.cn/posts/rust-diesel/

