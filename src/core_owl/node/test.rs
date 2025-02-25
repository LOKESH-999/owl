
#[test]
pub fn empty_line1(){
    let mut e_list = super::empty_line::EmptyMap::new();
    for _ in 0..65521{
        let len = e_list.get_empty_count();
        let idx = e_list.get_empty_idx();
        assert_eq!(len ,idx + 1);
        // println!("{},{len}",idx);
        // break;
    }
}