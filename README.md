## Java 重构中

多平台直播源地址获取, 主播开播提醒, 弹幕支持, 支持插件

### 欢迎大佬们 Fork PR 萌新练手小项目 求轻喷

## 路线
- [ ] API支持
- [ ] 多直播平台
- [ ] Vue前端
- [ ] 多SQL支持
  - [ ] MariaDB
  - [ ] Mysql
  - [ ] Mongodb
- [ ] 自动录制


### 目前设计

- 插件化
- 前后端分离

### SBtream-Server：

1. 作为一个 `Spring Boot`  项目 只对外提供 `RestFul`  接口

2. 插件化 使用 jar 作为插件格式 定义统一的 接口 使用 `Bull` 接口 定义统一结构

3. 提供 `POST`  `/{pid}/{rid}` 来接受请求  返回统一的请求体

    1. 扫描插件目录 使用 `Bull` 生成相应配置文件 `json#1`  包含

       ```json
       {
           "code": 0,
           "msg": "消息",
           "data": {
               "pid": "对应平台代号",
               "file": "文件地址",
               "class": "方法名"
           }
       }
       ```



2. `pid` 作为 作为 在调用服务时 在 `json#1` 中查找对应类，进行请求

3. 返回为标准返回体  （可能后续会添加上弹幕地址 再说吧）

   ```json
   {
       "code": 0,
       "msg": "消息",
       "data": {
           "url": "真实地址",
           "durl": "弹幕地址"
       }
   }
   ```





4. 提供平台列表的信息  提供 `POST`  `/plist` 来接受请求  返回统一的请求体

    1. 扫描插件目录 使用 `Bull` 来生成 `json`

       ```json
       {
           "code": 0,
           "msg": "消息",
           "data": [
               {
                   "pid": "平台代号",
                   "name": "直播平台的简称",
                   "url": "直播平台的官网",
                   "icon": "直播平台图标",
                   "mMan": "维护人的名字",
                   "mManEmail": "维护人的邮箱",
                   "msg": "维护人的公告信息",
                   "updateUrl": "维护更新地址",
                   "version": "当前版本"
               },{
                   "pid": "",
                   "name": "",
                   "url": "",
                   "icon": "",
                   "mMan": "",
                   "mManEmail": "",
                   "msg": "",
                   "updateUrl": "",
                   "version": ""
               }
           ]
       }
       ```



2.

5. 可以本地运行 也可以在服务器上暴露出来

6. 可以后续加入订阅系统 对应的用户 可以有不同的订阅

#### 接口

1. 更新插件列表 接口`PATCH` `/plist`

    - 入参 无

    - 出参

      ```json
      {
          "code": 0,
          "msg": "消息"
      }
      ```



2. 获取 直播地址 接口 `POST` `/{pid}/{rid}`

    - 入参  `pid`   `rid`

    - 出参

      ```json
      {
          "code": 0,
          "msg": "消息",
          "data": {
              "rurl": "真实地址",
              "durl": "弹幕地址"
          }
      }
      ```



3. 获取 平台列表 接口 `POST` `/plist`

    - 入参 无

    - 出参

      ```json
      {
          "code": 0,
          "msg": "消息",
          "data": [
              {
                  "pid": "平台代号",
                  "name": "直播平台的简称",
                  "icon": ""
              },{
                  "pid": "",
                  "name": "",
                  "icon": ""
              } 
          ]
      }
      ```

4. 获取 平台详情 接口 `POST` `/view/{pid}`

    - 入参 `pid`

    - 出参

      ```json
      {
          "code": 0,
          "msg": "消息",
          "data":
              {
                  "name": "直播平台的简称",
                  "url": "直播平台的官网",
                  "icon": "直播平台图标",
                  "needCookie": "是否需要 Cookies 来获取最高清的资源",
                  "mMan": "维护人的名字",
                  "mManEmail": "维护人的邮箱",
                  "msg": "维护人的公告信息",
                  "updateUrl": "维护更新地址",
                  "version": "当前版本"
              }
      }
      ```



5.

6. 多服务端 订阅？



### SBtream-Pulgin：

1. 只对接口做要求

2. 返回 对应平台的元数据

    1. 元数据

       ```json
       {
           "code": 0,
           "msg": "消息",
           "data":[
               {
                   "pid": "平台代号",
                   "name": "直播平台的简称",
                   "url": "直播平台的官网",
                   "icon": "直播平台图标",
                   "needCookie": "是否需要 Cookies 来获取最高清的资源",
                   "mMan": "维护人的名字",
                   "mManEmail": "维护人的邮箱",
                   "msg": "维护人的公告信息",
                   "updateUrl": "维护更新地址",
                   "version": "当前版本"
               }, {
                   "pid": "平台代号",
                   "name": "直播平台的简称",
                   "url": "直播平台的官网",
                   "icon": "直播平台图标",
                   "mMan": "维护人的名字",
                   "mManEmail": "维护人的邮箱",
                   "msg": "维护人的公告信息",
                   "updateUrl": "维护更新地址",
                   "version": "当前版本"
               }
       ]
       }
       ```

    2. 直播源

       ```json
       {
           "code": 0,
           "msg": "消息",
           "data": {
               "rurl": "真实直播地址",
               "durl": "弹幕地址"
           }
       }
       ```







### SBtream-Desktop:

1. 尝试使用 Vue+Electron 来构建 桌面系统
2. 使用 `SBtream-Server` 作为后端程序
3. 支持自定义服务端 即 使用第三方运行 `SBtream-Server`  提供的 api 接口

#### 接口：

1. 获取 全部订阅 接口 `POST` `/sub`

    - 入参 无

    - 出参

      ```json
      {
          "code": 0,
          "msg": "消息",
          "data":[
          {
              "pid": "平台代码",
              "sub": [
                  {
                      "rid": "房间号",
                      "name": "主播名", 
                      "star": "是否关注 （开播提醒）"
                  },{
                      "rid": "",
                      "name": "",
                      "star": ""
                  }
              ]
          },{
              "pid": "", 
              "sub": [
                  {
                      "rid": "",
                      "name": "",
                      "star": ""
                  },{
                      "rid": "",
                      "name": "",
                      "star": ""
                  }
              ]
          }
      ]
      }
      ```





2. 添加 订阅 接口 `PATCH` `/sub`

    - 入参

      ```json
      {
          "pid": "平台代号",
          "rid": "房间号",
          "name": "主播名",
          "star": "是否关注 （开播提醒）"
      }
      ```



- 出参

  ```json
  {
      "code": 0,
      "msg": "消息"
  }
  ```



3. 删除 订阅 接口 `DELET` `/sub`

    - 入参

      ```json
      {
          "pid": "平台代号",
          "rid": "房间号"
      }
      ```



- 出参

  ```json
  {
      "code": 0,
      "msg": "消息"
  }
  ```



4. 更新 订阅 接口

    - 入参

      ```json
      {
          "pid": "平台代号",
          "rid": "房间号",
          "name": "主播名",
          "star": "是否关注 （开播提醒）"
      }
      ```



- 出参

  ```json
  {
      "code": 0,
      "msg": "消息"
  }
  ```

     







