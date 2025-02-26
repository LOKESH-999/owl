
#[test]
pub fn empty_line1_idx_get(){
    let mut e_list = super::empty_line::EmptyMap::new();
    for _ in 0..65521{
        let len = e_list.get_empty_count();
        let idx = e_list.get_empty_idx();
        assert_eq!(len ,idx + 1);
        // println!("{},{len}",idx);
        // break;
    }
}

#[test]
pub fn empty_line2_idx_return(){
    let mut e_list = super::empty_line::EmptyMap::new();
    for _ in 0..65521{
        let len = e_list.get_empty_count();
        let idx = e_list.get_empty_idx();
        assert_eq!(len ,idx + 1);
    }
    for idx in 0..65521{
        e_list.return_free_idx(idx);
        let len = e_list.get_empty_count();
        assert_eq!(len ,idx + 1);
    }
}