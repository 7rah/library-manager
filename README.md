# 运行

```shell
./run.sh
```

# 尚未实现的功能

* 搜索借阅记录
* 搜索归还记录
* 搜索用户（用户管理页面）

# 数据库

database 支持 sqlite 或 mysql，默认使用 sqlite in memory

若要使用 mysql，修改 config.toml，取消 mysql 的注释，并注释掉 sqlite

使用 docker 快速设置 mysql

```shell
docker run -p 3306:3306 --name mysql -e MYSQL_ROOT_PASSWORD=123456 -d mysql
```