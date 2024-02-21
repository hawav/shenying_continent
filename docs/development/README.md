# 开发说明

## 项目结构

项目源码处在 `src` 下

```
area # 按区域区分代码逻辑
  hometown # 家园区域逻辑
  slime_plain # 史莱姆平原区域逻辑

creature # 游戏生物
  mod.rs # 生物建造函数
  type.rs # Creature 结构体定义
material # 游戏材料
  mod.rs # 材料建造函数
  type.rs # Material 结构体定义
weapon # 游戏武器
  mod.rs # 武器建造函数
  type.rs # Weapon、Quality 结构体定义

pk.rs # 战斗逻辑
lib.rs # 游戏通用逻辑

main.rs # cli 游戏运行环境

syscall # 对外唯一交互接口
  mod.rs # 根据运行环境使用对应实现
  std.rs # std 运行环境交互接口实现

```
