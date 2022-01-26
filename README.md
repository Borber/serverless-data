# Serverless-Data

在使用 `vercel` 进行 `serverless function` 开发时, 需要进行数据持久化的操作, 目前我的需求有
1. 对调用`api`进行次数统计

`vercel`实现`AWS Lambda`, 不支持 `Stream Reaponse`, 一旦返回数据将直接关闭, 不会继续执行后台任务, 这使得如果直接在`api`请求中操作数据库大大增加的响应时间, 据我的实践会从平均 `300ms` 增长到 `3s`

这是无法接受的, 所以另写一个专门处理数据库操作的 `serverless function` 服务来额外处理

逻辑:

![逻辑图](https://i.pstorage.space/i/gy59z80k/original_Untitled_Diagram.drawio_%283%29.png)

只对只需要执行操作, 不需要结果的操作适用

环境变量设置 `AUTH_CODE` 来认证
