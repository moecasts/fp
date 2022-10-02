# fp

**小鹤音形编码查询工具**

---

该工具使用 rust 开发，目前只支持通过源代码构建使用。

## 依赖
- rust

## 安装方法

```bash
# 克隆仓库
git clone https://github.com/moecasts/fp.git

# 进入项目目录
cd fp

# 构建
cargo build

# 软连接到 bin 目录
sudo ln -s ${PWD}/target/debug/fp /usr/local/bin/fp
```

## 使用方法

目前只支持单字查询

```bash
# 如果已经软连到 bin 目录的话（或者添加到 PATH 环境变量中)
fp 字

# 可以看到输出
字：    zi zibz*
===============================================
拆　分：  宀  子
首　末：  宀  子
编  码：  b  z
```
