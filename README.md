# idrs

>   对 [douyasi/identity-card](https://github.com/douyasi/identity-card) Rust 的语言实现版本。

### 编译运行

```shell
git clone https://github.com/ycrao/idrs
cd idrs
cp .env.example .env 
vim .env 
// 修改配置中 SQLITE_DB_PATH 值，执向项目数据库文件（请使用绝对路径）
// modify SQLITE_DB_PATH value pointed to your `project/db/id.sqlite` file (using absolute path)
cargo build
cargo run
```

### 结果示例

```shell
please input id the identity card (eg 42032319930606629x ):

empty identity card, give you a default case: 42032319930606629x
----------
identity card number: 42032319930606629x
validation passed: true
constellation: 双子座
gender: m
birthday: 19930606
age: 24
area: Area { status: true, result: "湖北省 十堰市竹山县", province: "湖北省", city: "十堰市", county: "竹山县", using: 1 }
```


### 其它实现版本

- PHP实现版本 [douyasi/identity-card](https://github.com/douyasi/identity-card)
- Node/Javascript实现版本 [id.js](https://github.com/ycrao/id.js)

### 联系作者

>   Email: raoyc2009#gmail.com （请修改改`#` 为`@`）  
>   QQ群：260655062  
