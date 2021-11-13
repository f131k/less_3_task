///
/// Объект для реализации обобщенной очереди
///
pub struct Queue<T> {
    pub queue: Vec<T>,
}

// Реализация методов очереди
impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    ///
    /// Добавление элемента в конец очереди
    ///
    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    ///
    /// Вытягивание первого элемента из очереди
    ///
    pub fn dequeue(&mut self) -> Option<T> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }

    ///
    /// Проверка на пустоту очереди
    ///
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
