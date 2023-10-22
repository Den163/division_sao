use std::{alloc::Layout, usize};

use paste::paste;

pub trait RefsTuple<'a> {
    
}

pub trait SaoTuple {
    type Tuple;
    type PtrsTuple;
    type PtrsTupleMut: SaoPtrsTupleMut;
    type RefsTuple<'a>;
    type RefsTupleMut<'a>;
}

pub trait SaoPtrsTupleMut {
    fn drop_ptrs(&mut self, len: usize);
}

pub struct SaoVec<T: SaoTuple> {
    data: T::PtrsTupleMut,
    len: usize,
    capacity: usize,
}

macro_rules! sao_tuple_impl {
    ($($T:ident),*) => {
        impl<$($T: 'static + Sized,)*> SaoPtrsTupleMut for ($(*mut $T,)*) {
            fn drop_ptrs(
                &mut self,
                size: usize
            ) {
                let ($( paste!([<data_$T:lower>]) ),*) = *self;

                unsafe {
                    $(
                        std::alloc::dealloc(
                            paste!([<data_$T:lower>]) as *mut u8,
                            arr_layout::<$T>(size)
                        );
                    )*
                }
            }
        }

        impl<$($T: 'static + Sized,)*> SaoTuple for ($($T,)*) {
            type Tuple = ($($T),*);
            type PtrsTuple = ($(*const $T,)*);
            type PtrsTupleMut = ($(*mut $T,)*);
            type RefsTuple<'a> = ($(&'a $T,)*);
            type RefsTupleMut<'a> = ($(&'a mut $T,)*);
        }

        impl<$($T: 'static,)*> SaoVec<($($T,)*)> {
            pub fn new() -> SaoVec<($($T),*)> {
                Self {
                    data: ($(std::ptr::null_mut::<$T>(),)*),
                    len: 0,
                    capacity: 0
                }
            }

            pub fn with_capacity(capacity: usize) -> SaoVec<($($T),*)> {
                Self {
                    data: ($( unsafe {
                        std::alloc::alloc(arr_layout::<$T>(capacity)) as *mut $T
                    } ),*),
                    len: 0,
                    capacity: capacity
                }
            }

            pub fn reserve(&mut self, capacity: usize) {
                if capacity <= self.len {
                    return;
                }

                self.increase_capacity(capacity);
            }

            pub fn push(&mut self, ($( paste!([<input_$T:lower>]), )*): ($($T,)*)) {
                self.check_grow();

                let ($( paste!([<data_$T:lower>]) ),*) = self.data;

                unsafe {
                    ($(
                        *paste!([<data_$T:lower>]).add(self.len) =
                            paste!([<input_$T:lower>])
                    ,)*);
                }

                self.len += 1;
            }

            pub fn remove(&mut self, index: usize) {                
                let last_index = self.len as isize - 1;
                let len_diff_i = last_index - index as isize;

                if len_diff_i < 0 {
                    if last_index < 0 {
                        panic!("The SAO is empty. Nothing to remove with index {index}");
                    }

                    panic!(
                        "Remove index is out of bounds. It should be between 0 and {last_index}, but it's {index}");
                }
                if len_diff_i > 0 {
                    let ($( paste!([<data_$T:lower>]) ),*) = self.data;
                    let len_diff_u = self.len - 1 - index;

                    $(
                        unsafe {
                            let ptr = paste!([<data_$T:lower>]).add(index);
                            ptr.add(1).copy_to(ptr, len_diff_u);
                        }
                    )*;
                }

                self.len -= 1;
            }

            pub fn swap_remove(&mut self, index: usize) {
                let ($( paste!([<data_$T:lower>]) ),*) = self.data;
                let last_index = self.len - 1;
                if (index != last_index) {
                    ($( unsafe {
                        let base_ptr = paste!([<data_$T:lower>]);
                        base_ptr.add(index).copy_from(base_ptr.add(last_index), 1);
                    } ),*);
                }

                self.len -= 1;
            }

            pub fn swap(&mut self, left_index: usize, right_index: usize) {
                let ($( paste!([<data_$T:lower>]) ),*) = self.data;
                ($( unsafe {
                    let base_ptr = paste!([<data_$T:lower>]);
                    base_ptr.add(left_index).swap(base_ptr.add(right_index));
                } ),*);
            }
 
            fn check_grow(&mut self) {
                if (self.len < self.capacity) {
                    return;
                } 

                if (self.capacity == 0) {
                    self.data = ($( unsafe {
                        std::alloc::alloc(arr_layout::<$T>(1)) as *mut $T 
                    },)*);

                    self.capacity = 1;
                } else {
                    self.increase_capacity(self.capacity * 2);
                }
            }

            fn increase_capacity(&mut self, new_capacity: usize) {
                let mut prev_ptrs = self.data;
                let ($( paste!([<prev_data_$T:lower>]) ),*) = prev_ptrs;

                self.data = unsafe { ( $(
                    std::alloc::alloc(
                        arr_layout::<$T>(new_capacity)
                    ) as *mut $T
                ),* ) };

                let ($( paste!([<new_data_$T:lower>]) ),*) = self.data;
                ($(unsafe {
                    paste!([<new_data_$T:lower>]).copy_from_nonoverlapping(
                        paste!([<prev_data_$T:lower>]), 
                        self.capacity
                    )
                }),*);

                prev_ptrs.drop_ptrs(self.capacity);
                self.capacity = new_capacity;
            }

            pub fn as_refs_tuple(&self) -> ($(& [$T],)*) {
                let ($( paste!([<$T:lower>]) ),*) = self.data;
                ($( unsafe {
                    std::slice::from_raw_parts(paste!([<$T:lower>]), self.len)
                } ),*)
            }

            pub fn as_refs_tuple_mut(&self) -> ($(&mut [$T],)*) {
                let ($( paste!([<$T:lower>]) ),*) = self.data;
                ($( unsafe {
                    std::slice::from_raw_parts_mut(paste!([<$T:lower>]), self.len)
                } ),*)
            }

            pub fn len(&self) -> usize {
                self.len
            }

            pub fn capacity(&self) -> usize {
                self.capacity
            }

            pub fn at(&self, index: usize) -> ($(& $T,)*) {
                let ($( paste!([<$T:lower>]) ),*) = self.data;
                unsafe {
                    ($(
                        & *paste!([<$T:lower>]).add(index) as & $T
                    ,)*)
                }
            }

            pub fn at_mut(&mut self, index: usize) -> ($(&mut $T,)*) {
                let ($( paste!([<$T:lower>]) ),*) = self.data;
                unsafe {
                    ($(
                        &mut *paste!( [<$T:lower>] ).add(index) as &mut $T
                    ,)*)
                }
            }
        }
    }
}

impl<T: SaoTuple> Drop for SaoVec<T> {
    fn drop(&mut self) {
        self.data.drop_ptrs(self.capacity);
    }
}

#[inline]
unsafe fn arr_layout<T>(size: usize) -> Layout {
    unsafe {
        let a = Layout::array::<T>(size).unwrap_unchecked();
        a
    }
}


sao_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
sao_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
sao_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
sao_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
sao_tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7);
sao_tuple_impl!(T0, T1, T2, T3, T4, T5, T6);
sao_tuple_impl!(T0, T1, T2, T3, T4, T5);
sao_tuple_impl!(T0, T1, T2, T3, T4);
sao_tuple_impl!(T0, T1, T2, T3);
sao_tuple_impl!(T0, T1, T2);
sao_tuple_impl!(T0, T1);
