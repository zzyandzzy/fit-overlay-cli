pub struct NumberedFrame {
    pub frame_num: u32,
    pub image: Vec<u8>,
}

// 实现 Eq。因为 NumberedFrame 需要能够作为一个二进制堆的元素，
// 它必须实现 Eq 和 Ord。
impl Eq for NumberedFrame {}

// 实现 PartialEq，这样我们就可以比较两个 NumberedFrame 是否相等。
// 这里我们只关心 frame_num 是否相等。
impl PartialEq for NumberedFrame {
    fn eq(&self, other: &Self) -> bool {
        self.frame_num == other.frame_num
    }
}

// 实现 PartialOrd。这是用于二进制堆的排序逻辑。
// 我们使用 frame_num 来决定顺序。
impl PartialOrd for NumberedFrame {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// 实现 Ord。这是实际的比较逻辑，决定了堆中元素的顺序。
// 我们希望具有较小 frame_num 的元素先出队。
impl Ord for NumberedFrame {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 注意我们使用了 reverse，因为二进制堆是一个最大堆，
        // 但我们需要的是最小 frame_num 的元素先出队。
        other.frame_num.cmp(&self.frame_num)
    }
}
