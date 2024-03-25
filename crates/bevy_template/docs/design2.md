## 这是什么

一种可以添加Template，Bundle，Component，Info的模板类型，总是由后覆盖前

## 资产示例

文件：`template.RON`,`base.RON`

### V1

```RON
// 一个简单的模板
[
    ("base.RON"), //Template，相当于继承
    Visiblity(visible:false), //Componet
    SpriteBundle(
        texture:("sprite1.png")//Handle<Image>
    ), //Bundle
    Image("sprite2.png"), //Asset，会直接加载为Handle<A>，下方的项可以覆盖上方项的组件
    HealthInfo(max_health:100), //Info，Info会产生特殊的Component
]
```

### V2

```RON
(
    dep:["base.RON"],
    content:{
        Bundle:(
            comp1:("data1"),
        ),
        Info:(),
        Component:(),
    }
)
```

##     

## 清单

1. 内置Bundle没有实现反射

   解决办法：不使用内部Bundle
2. 基于Default的反射类型反序列化以实现默认值

   解决办法：内置类型不使用默认值，自定义类型使用serde提供的default
3. 循环依赖？
4. 使用后处理？