## 作业一：

从安全性角度：

链上产生的随机数是不安全的，在前面的Kitties示例中，使用了Randomness::random_seed()，根据文档，在同一个区块内多次调用随机种子，会返回相同的值。而且节点A和节点B在运行同一个块的链上随机数代码产生的随机数是一样的。

The `random_seed` method takes no parameters and return a raw random value. If you call this method multiple times in a block, it returns the same value each time. Therefore, in most cases, you shouldn't use this method directly.

This type of randomness performs well but isn't secure. You should only use this pallet in applications with low security  requirements or when testing randomness-consuming applications. You shouldn't use this pallet in a production environment.

链下工作机可以使用安全的随机数生成算法，用一个安全的本地熵源（entropy）生成随机数，所以相比链上随机数，是更安全的。

从产生的角度：

在链上生成的随机数，节点生成链上随机数有一份证明（签名），可以供别人验证。而链下随机数没有办法很好地提供证明，因为是用本地的随机种子。



## 作业二：使用 offchain indexing 向 offchain storage 写入数据
![write data from on-chain to offchain storage by using offchain indexing](https://github.com/eventXhorizon/6th_substrate_adv_homework/assets/127678969/f748b603-5209-4888-a0b9-22e5cc325967)



## 作业三：获取了作业二写入 offchain storage 数据
![frontend1](https://github.com/eventXhorizon/6th_substrate_adv_homework/assets/127678969/74022a8c-53f8-49ce-a017-02ed8fd333f8)

![frontend2](https://github.com/eventXhorizon/6th_substrate_adv_homework/assets/127678969/d3dde25c-5a7b-4006-ac7d-80093061cf74)



## 作业四：从外部获取天气数据，然后从 offchain worker 向链上发起带签名负载的不签名交易
![get_weather_info_and_unsigned_with_payload](https://github.com/eventXhorizon/6th_substrate_adv_homework/assets/127678969/5a5cba33-0e23-41a5-ab96-3378da805015)

