### fltk-rs

获取到的是逻辑坐标系 (可以直接使用)

- 问题
    - 多屏幕情况下, 其他屏幕的窗口会先使用**调用程序所在的屏幕**的`scale_factor`,
      然后闪烁一下变成目标屏幕的`scale_factor`. (
      不知道是windows的问题还是fltk-rs的问题)
    - 多屏幕情况下, 绘制坐标会受到**调用程序所在的屏幕**的`scale_factor`影响,

### screenshots

烂



