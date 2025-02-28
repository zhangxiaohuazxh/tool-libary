pub trait List<T> {
    /**
    ### 获取List中元素的个数
    #### @return 元素个数
    */
    fn size(&self) -> usize;

    /**
    ## 查找List中是否已经包含target元素
    #### @param target 要查找的目标值
    #### @return 查找结果
    */
    fn contains(&self, target: T) -> bool;
}
