-- Your SQL goes here
CREATE TABLE `posts`
(
    `id`        bigint       NOT NULL AUTO_INCREMENT COMMENT 'ID',
    `title`     varchar(256) NOT NULL COMMENT '标题',
    `body`      text         NOT NULL COMMENT '内容',
    `published` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否发布',
    PRIMARY KEY (`id`)
);