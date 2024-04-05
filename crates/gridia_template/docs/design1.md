# 模板

## 这是什么

ECS系统中Component的定位是运行时状态，对于每个实体，其运行时状态都是独立的

需要一种可以共享的数据，通过修改这个数据可以影响所有使用这个数据生成的实体

## 参考

1. OpenRA
2. 冰汽时代

## 概念

模板（Info）是一种Asset，以Handle的形式共享

每个模板都有对应的Component，Component会持有模板的Handle

模板配置文件中使用模板的组合一个Entity的定义，系统会加载该配置并得到一个模板Entity

Entity

## 问题

如果在运行时

## 示例

```json5
{
}
```

```rust

trait Template(){}


```
