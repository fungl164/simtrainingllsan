# 环境搭建

目前编译时必须安装rust的nightly版本;

安装postgresql数据库;

数据库设置具体看这里：http://diesel.rs/guides/getting-started/

# 测试restful服务的方法（具体restful的URL配置参见examples/curl_test_script.txt）

先打开一个终端，运行cargo run --example testrouter2

然后再打开一个终端，用examples/curl_test_script.txt中每一行的命令进行测试

```bash
root@root-linux:~/rustprojs/simtraining$ multirust show-default
multirust: default toolchain: nightly
multirust: default location: /home/root/.multirust/toolchains/nightly

rustc 1.11.0-nightly (6e00b5556 2016-05-29)
cargo 0.11.0-nightly (3ff108a 2016-05-24)

root@root-linux:~/rustprojs/simtraining$ cargo run -j 8 --example testrouter2
   Compiling simtraining v0.1.0 (file:///home/root/rustprojs/simtraining)
     Running `target/debug/examples/testrouter2`

```

```bash
root@root-linux:~$ curl -X GET 'http://localhost:3000/api/v1/trainingsession'
[{"id":0,"sec":0,"nsec":0,"name":"0","xitong_id":0,"admin_uid":"0","users_uid":["0","1"],"actions_id":[0,1],"mode":"0","state":"0","sec_duration":0,"nsec_duration":0,"score_op_order":0,"score_op_correct":0,"score_op_duration":0,"score":0},{"id":1,"sec":0,"nsec":0,"name":"0","xitong_id":0,"admin_uid":"0","users_uid":["0","1"],"actions_id":[0,1],"mode":"0","state":"0","sec_duration":0,"nsec_duration":0,"score_op_order":0,"score_op_correct":0,"score_op_duration":0,"score":0},{"id":3,"sec":0,"nsec":0,"name":"0","xitong_id":0,"admin_uid":"0","users_uid":["0","1"],"actions_id":[0,1],"mode":"0","state":"0","sec_duration":0,"nsec_duration":0,"score_op_order":0,"score_op_correct":0,"score_op_duration":0,"score":0}]

```
# 构建测试程序
环境什么的搭建好了后，使用以下命令构建并运行测试服务器

```bash
cargo clean
cargo update
cargo build
cargo build --example simserver
cargo run --example simserver
```
可通过使用get方法访问`http://localhost:3000/api/v1/xitong`来获取当前运行系统的所有json数据，可通过使用get方法访问`http://localhost:3000/api/v1/zhiling`来获取发送指令的json数据格式示例，发送指令请采用post方法访问`http://localhost:3000/api/v1/zhiling/:id`，其中id为系统id，暂时规定为0，json数据格式与'./src/xitong.rs'中的`XiTong`结构体结构相同，如果想让json数据格式更美观一些，可使用`http://json.bloople.net/`将json转换为可嵌套的表格。
目前发送指令还没有响应，具体代码正在紧张构建中。
