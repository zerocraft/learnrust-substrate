# learnrust-substrate

## Advanced course

### l6 benchmark & online

- benchmarking [代码](la1/pallets/poex/src/benchmarking.rs)

- weights [代码](la1/pallets/poex/src/weights.rs)

- 创建 chain spec 文件
    ``` bash
    # chain spec
    ./target/release/node-template build-spec > chain_spec.json
    # chain spec raw
    ./target/release/node-template build-spec --chain=chain_spec.json --raw > chain_spec_raw.json
    ```

### l5 smart contracts

- erc20 合约代码 [代码](la5/erc20/lib.rs)

- contracts-node 启动 [运行结果](la5/assets/r1.png)

- 使用 contracts-ui 运行查询 [运行结果](la5/assets/r2.png)

- 使用 contracts-ui 运行转账 [运行结果](la5/assets/r3.png)

### l4 Off-chain Worker

- offchain_index 写入数据 [代码](la4/pallets/ocwx/src/lib.rs#L159)

- Worker 获取 Offchain Storage 数据 [运行结果](la4/assets/r1.png)

- Frontend 获取 Offchain Storage 数据 [运行结果](la4/assets/r2.png)

- Http 获取价格并从OCW中向链上发起带签名负载的不签名交易 [代码](la4/pallets/ocwx/src/lib.rs#L318)

- Console 显示运行状态的情况 [运行结果](la4/assets/r3.png)

- Frontend 查询交易数据 [运行结果](la4/assets/r4.png)

### l3 Substate Kitties - 2

- kitties 方法完善 [题4 - 代码 1](la1/pallets/kittiesx/src/lib.rs#L238)

- kitties 单元测试 [题4 - 代码 2](la1/pallets/kittiesx/src/tests.rs#L168)

- runtime 升级 [题5 - 代码](la1/pallets/kittiesx/src/migrations/v2.rs)

- Frontend 查看升级前数据 [运行结果 升级前](la1/assets/r6.png)

- Frontend 查看升级后数据 [运行结果 升级后](la1/assets/r7.png)

### l2 Substate Kitties - 1

- kitties pallet [题4 - 代码](la1/pallets/kittiesx/src/lib.rs)

- kitties 单元测试 [运行结果 1](la1/assets/r2.png)

- Node 编译运行 [运行结果 2](la1/assets/r3.png)

- Frontend kitties 交易 [运行结果 3](la1/assets/r4.png)

- 获取 event 的单元测试 [题5 - 代码](la1/pallets/kittiesx/src/tests2.rs)

- event 单元测试输出 [运行结果](la1/assets/r5.png)

### l1 Proof of Existence

- 测试用例 [题4 - 代码](la1/pallets/poex/src/lib.rs)

- 测试输出 [运行结果](la1/assets/r1.png)

## Basic Introduction

### l6

- [题4 - 代码](l6/main.ts)

### l5

- [题5,6 - 代码](l5/pallets/poe/src/lib.rs)

- [运行结果 1](l5/assets/r1.png)

- [运行结果 2](l5/assets/r2.png)

- [运行结果 3](l5/assets/r3.png)

### l4

- [题5 - 代码](l4/src/uutils/traffic_light.rs)

- [题6 - 代码](l4/src/uutils/tools.rs)

- [题7 - 代码](l4/src/uutils/graph.rs)

### l3

- [题5 - 代码](l3/src/main.rs)

- [运行结果](l3/assets/r1.png)