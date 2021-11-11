#  基于substrate实现的链上存证功能

##  1.创建存证

-  初始链状态中的存储对应的存证为none

  ![avatar](./img/init.png)

-  创建存证，再次查看，存证上链

  ![avatar](./img/create.png)

##  2.撤销存证

- 执行poe模块的revoke撤销函数，撤销存证

  ![avatar](./img/revoke.png)

- 再次查看链上存储，发现为none，存证撤销成功

  ![avatar](./img/init.png)

##  3.转移存证

- 执行poe的transfer函数，转移存证

  ![avatar](./img/alice.png)
  
  ![avatar](./img/alictobob.png)

- 检查存证发现alice的存证已被转移到bob账户
  
  ![avatar](./img/bob.png)
