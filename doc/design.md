# design

## 数据库表

`users` 用户表
| name     | type        | description |
| -------- | ----------- | ----------- |
| id       | bigint      | 主键        |
| username | varchar(50) | 用户名      |

`conversations` 会话表
| name  | type         | description                          |
| ----- | ------------ | ------------------------------------ |
| id    | bigint       | 主键                                 |
| types | integer      | 会话类型（0: 单人会话，1: 群组会话） |
| name  | varchar(100) | 会话名称                             |

`messages` 消息表
| name            | type     | description                                       |
| --------------- | -------- | ------------------------------------------------- |
| id              | bigint   | 主键                                              |
| sender_id       | bigint   | 用户ID                                            |
| conversation_id | bigint   | 会话ID                                            |
| created_at      | datetime | 消息创建时间                                      |
| message_type    | integer  | 消息类型（0: 文本消息，1: 图片消息，2: 语音消息） |
| content         | text     | 消息内容                                          |

`conversation_members` 会话成员表
| name            | type    | description                    |
| --------------- | ------- | ------------------------------ |
| conversation_id | bigint  | 会话ID                         |
| user_id         | bigint  | 用户ID                         |
| role            | integer | 角色（0: 普通成员，1: 管理员） |

