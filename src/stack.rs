///
/// Объект для реализации стека
///
pub struct Stack<T> {
    pub stack: Vec<T>,
}

// Реализация методов стека
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    ///
    /// Получение последнего объекта с его выталкиванием из стека
    ///
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    ///
    /// Размещение объекта на верхушке стека
    ///
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    ///
    /// Проверка на пустоту стека
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    ///
    /// Получение последнего объекта без выталкивания его из стека
    ///
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}
