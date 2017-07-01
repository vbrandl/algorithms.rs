use std::sync::Arc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Default::default()
    }

    /// Appends the current list to `elem` and returns the new list.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::persistent_list::List;
    ///
    /// let list = List::new().append(1).append(2).append(3);
    /// assert_eq!(list.head(), Some(&3));
    /// ```
    pub fn append(&self, elem: T) -> List<T> {
        Self {
            head: Some(Arc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    /// Removes the first element of a list and returns the resulting list.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::persistent_list::List;
    ///
    /// let list = List::new().append(1).append(2).append(3).tail();
    /// assert_eq!(list.head(), Some(&2));
    /// ```
    pub fn tail(&self) -> List<T> {
        Self { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    /// Returns the first element of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::persistent_list::List;
    ///
    /// let list: List<i32> = List::new();
    /// assert_eq!(list.head(), None);
    ///
    /// let list = list.append(1);
    /// assert_eq!(list.head(), Some(&1));
    ///
    /// let list = list.tail();
    /// assert_eq!(list.head(), None);
    /// ```
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Converts the collection to an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::persistent_list::List;
    ///
    /// let list = List::new().append(1).append(2).append(3);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { head: None }
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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}
