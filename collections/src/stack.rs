use std::iter::IntoIterator;

pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(Stack<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Stack<T> {
    /// Creates a new stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::stack::Stack;
    ///
    /// let mut stack: Stack<i32> = Stack::new();
    ///
    /// assert_eq!(stack.pop(), None);
    /// ```
    pub fn new() -> Self {
        Default::default()
        // Self { head: None }
    }

    /// Adds a new element to the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    ///
    /// assert_eq!(stack.pop(), None);
    ///
    /// stack.push(1);
    /// stack.push(2);
    /// stack.push(3);
    ///
    /// assert_eq!(stack.pop(), Some(3));
    /// assert_eq!(stack.pop(), Some(2));
    ///
    /// stack.push(4);
    /// stack.push(5);
    ///
    /// assert_eq!(stack.pop(), Some(5));
    /// assert_eq!(stack.pop(), Some(4));
    ///
    /// assert_eq!(stack.pop(), Some(1));
    /// assert_eq!(stack.pop(), None);
    /// ```
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /// Removes the top element from the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    ///
    /// assert_eq!(stack.pop(), None);
    ///
    /// stack.push(1);
    /// stack.push(2);
    /// stack.push(3);
    ///
    /// assert_eq!(stack.pop(), Some(3));
    /// assert_eq!(stack.pop(), Some(2));
    ///
    /// stack.push(4);
    /// stack.push(5);
    ///
    /// assert_eq!(stack.pop(), Some(5));
    /// assert_eq!(stack.pop(), Some(4));
    ///
    /// assert_eq!(stack.pop(), Some(1));
    /// assert_eq!(stack.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }

    /// Returns a reference to the stacks top value.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    ///
    /// stack.push(5);
    /// stack.push(4);
    /// stack.push(3);
    ///
    /// assert_eq!(stack.peek(), Some(&3));
    ///
    /// stack.pop();
    ///
    /// assert_eq!(stack.peek(), Some(&4));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns a mutable reference to the stacks top value.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    ///
    /// stack.push(5);
    /// stack.push(4);
    /// stack.push(3);
    ///
    /// assert_eq!(stack.peek_mut(), Some(&mut 3));
    ///
    ///
    /// stack.pop();
    ///
    /// assert_eq!(stack.peek_mut(), Some(&mut 4));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    /// Creates an iterator over the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(1); stack.push(2); stack.push(3);
    ///
    /// let mut iter = stack.iter();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    /// Creates a mutable iterator over the stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(1); stack.push(2); stack.push(3);
    ///
    /// let mut iter = stack.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Transforms the stack into an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(1); stack.push(2); stack.push(3);
    ///
    /// let mut iter = stack.into_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(1));
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}
