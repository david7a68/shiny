use std::{iter::FromIterator, mem::MaybeUninit};

/// A fixed-capacity vector of T. Attempting to add elements beyond its capacity
/// will cause a panic.
pub struct ArrayVec<T, const N: usize> {
    // We just need the allocated space, don't really care about what's in it.
    array: MaybeUninit<[T; N]>,
    length: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    /// Create a new fized-capacity vector on the stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of elements in the vector.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Shorthand for `len() == 0`
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns true if the array has reached full capacity.
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// The statically determined capacity for the vector.
    pub fn capacity(&self) -> usize {
        N
    }

    /// Retrieves a pointer to the front of the reserved buffer. Only elements
    /// `0..len()` are guaranteed to have been initialized.
    pub fn as_ptr(&self) -> *const T {
        unsafe { (*self.array.as_ptr()).as_ptr() }
    }

    /// Retrieves a mutable pointer to the front of the reserved buffer. Only
    /// elements `0..len()` are guaranteed to have been initialized.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        unsafe { (*self.array.as_mut_ptr()).as_mut_ptr() }
    }

    /// Produces a slice spanning the entire vector.
    pub fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.as_ptr(), self.length as usize) }
        }
    }

    /// Produces a mutable slice spanning the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.is_empty() {
            &mut []
        } else {
            unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.length) }
        }
    }

    pub fn clear(&mut self) {
        for element in self.as_mut_slice() {
            unsafe {
                std::ptr::drop_in_place(element);
            }
        }
        self.length = 0;
    }

    /// Consumes the vector and calls a closure on every element, allowing it to
    /// drop at the end of the closure.
    pub fn empty<F>(mut self, mut f: F)
    where
        F: FnMut(T),
    {
        for i in 0..self.length {
            f(unsafe { std::ptr::read(self.as_mut_ptr().add(i)) });
        }
    }

    /// Pushes a new element to the back of the vector.
    ///
    /// # Panics
    /// This function will panic if the vector is at capacity.
    pub fn push(&mut self, value: T) {
        if self.length < N {
            unsafe {
                self.as_mut_ptr().add(self.length).write(value);
            }

            self.length += 1;
        } else {
            panic!("ArrayVec out of capacity");
        }
    }

    #[must_use]
    pub fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            unsafe {
                self.set_len(self.len() - 1);
                Some(std::ptr::read(self.as_ptr().add(self.len())))
            }
        } else {
            None
        }
    }

    /// Creates a by-reference iterator over the elements in the vector.
    #[must_use]
    #[allow(clippy::needless_lifetimes)] // Compiler gets into a cycle w/o parameters
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.as_slice().iter()
    }

    /// Sets the length of the vector to `length`.
    ///
    /// # Safety
    /// Make sure that the elements `0 .. length` have been initialized.
    /// Accessing uninitialized elements is undefined behavior.
    ///
    /// # Panics
    /// This function will panic if `length` is greater than `N`.
    pub unsafe fn set_len(&mut self, length: usize) {
        if length <= N {
            self.length = length;
        } else {
            panic!("attempted to set length on ArrayVec outside of bounds.");
        }
    }
}

impl<T, const N: usize> Default for ArrayVec<T, N> {
    #[must_use]
    fn default() -> Self {
        Self {
            array: MaybeUninit::uninit(),
            length: 0,
        }
    }
}

impl<T, const N: usize> Drop for ArrayVec<T, N> {
    fn drop(&mut self) {
        for element in self.as_mut_slice() {
            unsafe {
                std::ptr::drop_in_place(element);
            }
        }
    }
}

impl<T, const N: usize> std::ops::Deref for ArrayVec<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> std::ops::DerefMut for ArrayVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for ArrayVec<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a ArrayVec<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.as_slice().iter()
    }
}

impl<T, const N: usize> std::ops::Index<usize> for ArrayVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // let slice do the bounds checking for us
        &self.as_slice()[index]
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for ArrayVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // let slice do the bounds checking for us
        &mut self.as_mut_slice()[index as usize]
    }
}

impl<T, const N1: usize, const N2: usize> From<[T; N1]> for ArrayVec<T, N2>
where
    T: Copy,
{
    fn from(slice: [T; N1]) -> Self {
        assert!(N1 <= N2);
        let mut vec = Self::default();
        let vec_slice = {
            let ptr = vec.array.as_mut_ptr().cast();
            unsafe { std::slice::from_raw_parts_mut(ptr, N1) }
        };
        vec_slice.copy_from_slice(&slice);
        unsafe {
            vec.set_len(N1);
        }
        vec
    }
}

impl<T, const N: usize> FromIterator<T> for ArrayVec<T, N> {
    /// Creates a new `ArrayVec`, and fills it with values from the iterator.
    /// The `ArrayVec` will take as many elements as the iterator contains, up
    /// to N elements.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Self::default();

        let mut ptr = vec.array.as_mut_ptr().cast::<T>();
        let mut length = 0;

        let end = unsafe { ptr.add(N) };

        for v in iter {
            if ptr == end {
                break;
            }

            unsafe {
                ptr.write(v);
                ptr = ptr.add(1);
                length += 1;
            }
        }

        assert!(length <= N);
        unsafe {
            vec.set_len(length);
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_vec_init() {
        let mut vec = ArrayVec::<u32, 3>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 3);

        assert_eq!(vec.as_slice().len(), 0);
        assert_eq!(vec.as_mut_slice().len(), 0);
    }

    #[test]
    fn array_vec_push_drop() {
        static mut K: u32 = 0;

        #[derive(Debug)]
        struct T(u32);

        impl Drop for T {
            fn drop(&mut self) {
                unsafe { K += 1 };
            }
        }

        impl PartialEq<u32> for T {
            fn eq(&self, other: &u32) -> bool {
                self.0 == *other
            }
        }

        let mut vec = ArrayVec::<T, 4>::new();

        vec.push(T(0));
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.as_slice(), [0]);
        assert_eq!(vec.as_mut_slice(), [0]);

        vec.as_mut_slice()[0] = T(1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.as_slice(), [1]);
        assert_eq!(vec.as_mut_slice(), [1]);

        vec.push(T(2));
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.as_slice(), [1, 2]);
        assert_eq!(vec.as_mut_slice(), [1, 2]);

        std::mem::drop(vec);

        unsafe { assert_eq!(K, 3) };
    }

    #[test]
    fn array_vec_from_iter() {
        {
            // Saturating 0-sized array
            let vec = ArrayVec::<u32, 0>::from_iter(std::iter::repeat(100));

            assert_eq!(vec.len(), 0);
            assert_eq!(vec.len(), vec.capacity());
            assert_eq!(vec.as_slice(), []);
        }
        {
            // Saturating N-sized array
            let vec = ArrayVec::<u32, 4>::from_iter(std::iter::repeat(100));

            assert_eq!(vec.len(), 4);
            assert_eq!(vec.len(), vec.capacity());
            assert_eq!(vec.as_slice(), [100, 100, 100, 100]);
        }
        {
            // Underfilling N-sized array
            let vec = ArrayVec::<u32, 4>::from_iter(std::iter::once(100));

            assert_eq!(vec.len(), 1);
            assert_eq!(vec.as_slice(), [100]);
        }
    }
}
