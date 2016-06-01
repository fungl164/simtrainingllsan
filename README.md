注：目前编译时必须安装rust的nightly版本

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
