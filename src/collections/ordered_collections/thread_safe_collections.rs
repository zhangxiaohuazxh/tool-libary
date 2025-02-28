pub use crate::collections::base::List;

/**
### 写时复制的List实现,适用于读多写少的场景,数据量大的情况下会占用大量内存
*/
#[derive(Debug)]
pub struct CopyOnWriteList<T> {
    // 容量
    capacity: usize,
    // 存放的数据
    data: Vec<T>,
}

impl<T> List<T> for CopyOnWriteList<T> {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn contains(&self, target: T) -> bool {
        todo!()
    }
}

impl<T> CopyOnWriteList<T> {
    pub fn new() -> Self {
        Self {
            capacity: 3,
            data: vec![],
        }
    }

    /**
    ### 创建CopyOnWriteList<T>
    #### initial_capacity 初始化大小
    */
    pub fn new_with_initial_capacity(initial_capacity: usize) -> Self {
        Self {
            capacity: initial_capacity,
            data: vec![],
        }
    }

    /**
    ### 创建CopyOnWriteList<T>
    #### data 初始数据
    */
    pub fn new_with_initial_data(data: Vec<T>) -> Self {
        let capacity = data.len();
        let mut new_data = Vec::with_capacity(capacity);
        data.into_iter().for_each(|e| new_data.push(e));
        Self {
            capacity: Self::grow_length(capacity),
            data: new_data,
        }
    }
    /**
    ### 确定扩容的容量,保证容量始终是2的倍数
    #### capacity 当前List的容量
    */
    fn grow_length(capacity: usize) -> usize {
        if capacity % 2 == 0 {
            capacity * 2
        } else {
            (capacity + 1) * 2
        }
    }

    /**
    ### 扩容当前List
    */
    fn grow(&mut self) {
        self.capacity = Self::grow_length(self.capacity);
    }
}

#[cfg(test)]
mod thread_safe_collections_test {
    use crate::collections::base::List;
    use crate::collections::ordered_collections::thread_safe_collections::CopyOnWriteList;

    #[test]
    fn test_copy_on_write_list() {
        let list: CopyOnWriteList<i32> = CopyOnWriteList::new_with_initial_data(vec![1, 2, 3]);
        println!("list {:?}", list);
        let len = list.size();
        println!("len {}", len);
    }
}
