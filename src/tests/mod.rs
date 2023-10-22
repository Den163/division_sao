#[cfg(test)]
mod tests {
    use crate::SaoVec;

    #[test]
    pub fn new_will_return_empty_sao() {
        let sao = SaoVec::<(u32, u64)>::new();

        assert_eq!(sao.len(), 0);
        assert_eq!(sao.capacity(), 0);
    }

    #[test]
    pub fn push_add_new_element_and_increase_len_and_capacity() {
        let mut sao = SaoVec::<(u32, u64)>::new();

        sao.push((1, 2));

        assert_eq!(sao.len(), 1);
        assert_ne!(sao.capacity(), 0);
        assert_eq!(sao.at(0), (&1, &2));

        sao.push((3, 4));

        assert_eq!(sao.len(), 2);
        assert_eq!(sao.at(0), (&1, &2));
        assert_eq!(sao.at(1), (&3, &4));
    }

    #[test]
    pub fn remove_erase_element_and_shift_left_others() {
        let mut sao = SaoVec::<(u64, u32)>::new();
        sao.push((1, 2));
        sao.push((3, 4));
        sao.push((5, 6));

        sao.remove(1);

        assert_eq!(sao.len(), 2);
        assert_eq!(sao.at(1), (&5, &6));

        sao.remove(1);

        assert_eq!(sao.len(), 1);
        assert_eq!(sao.at(0), (&1, &2));

        sao.remove(0);
        assert_eq!(sao.len(), 0);
    }

    #[test]
    pub fn swap_remove_as_expected() {
        let mut sao = SaoVec::<(u64, u32)>::new();

        sao.push((1, 2));
        sao.push((3, 4));
        sao.push((5, 6));
        sao.push((7, 8));

        sao.swap_remove(1);

        assert_eq!(sao.len(), 3);
        assert_eq!(sao.at(1), (&7, &8));
    }

    #[test]
    #[should_panic]
    pub fn remove_should_panic_on_out_of_bounds_index() {
        let mut sao = SaoVec::<(u32, u64)>::new();
        sao.push((1, 2));

        sao.remove(1);
    }

    #[test]
    #[should_panic]
    pub fn remove_should_panic_on_empty_sao() {
        let mut sao = SaoVec::<(u32, u64)>::new();
        sao.remove(0);
    }
}
