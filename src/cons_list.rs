pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub fn head<T>(list: &List<T>) -> Option<&T> {
    match &list {
        List::Cons(v, _) => Some(v),
        List::Nil => None,
    }
}

pub fn tail<T>(list: &List<T>) -> Option<&T> {
    match list {
        List::Cons(v, l) => {
            let next = l.clone();
            match next.as_ref() {
                List::Cons(_,_) => tail(next.as_ref()),
                List::Nil => Some(v),
            }
        }
        List::Nil => None,
    }
}

pub fn get<T>(i: usize, list: &List<T>) -> Option<&T> {
    match list {
        List::Cons(v,l) => {
            if i <= 0 {
                Some(v)
            } else {
                get(i - 1, l.as_ref())
            }
        },
        List::Nil => None,
    }
}

pub fn len<T>(list: &List<T>) -> usize {
    match list {
        List::Cons(_, l) => 1 + len(l.as_ref()),
        List::Nil => 0,
    }
}

pub fn drop<T>(list: &List<T>) -> &List<T> {
    match list {
        List::Cons(_, l) => l.as_ref(),
        List::Nil => &List::Nil,
    }
}

mod tests {
    use super::*;

    #[test]
    fn should_create_a_cons_list() {
        let el_1 = Box::new(List::Cons(10, Box::new(List::Nil)));
        let el_2 = Box::new(List::Cons(20, el_1));

        let list = List::Cons(30, el_2);

        assert_eq!(3, len(&list))
    }

    #[test]
    fn should_return_head_of_list() {
        let el_1 = Box::new(List::Cons(10, Box::new(List::Nil)));
        let el_2 = Box::new(List::Cons(20, el_1));

        let list = List::Cons(30, el_2);

        assert_eq!(Some(30), head(&list).copied());
        assert_eq!(None, head::<i32>(&List::Nil));
    }

    #[test]
    fn should_return_tail_of_list() {
        let el_1 = Box::new(List::Cons(10, Box::new(List::Nil)));
        let el_2 = Box::new(List::Cons(20, el_1));

        let list = List::Cons(30, el_2);

        assert_eq!(Some(10), tail(&list).copied());
        assert_eq!(None, tail::<i32>(&List::Nil));
    }

    #[test]
    fn should_return_get_element_of_list_by_index() {
        let el_1 = Box::new(List::Cons(10, Box::new(List::Nil)));
        let el_2 = Box::new(List::Cons(20, el_1));

        let list = List::Cons(30, el_2);

        assert_eq!(Some(20), get(1, &list).copied());
        assert_eq!(None, get::<i32>(10, &List::Nil));
    }

    #[test]
    fn should_drop_head_of_list() {
        let el_1 = Box::new(List::Cons(10, Box::new(List::Nil)));
        let el_2 = Box::new(List::Cons(20, el_1));
        let list = List::Cons(30, el_2);

        let list_2 = drop(&list);
        assert_eq!(Some(20), head(list_2).copied());

        let list_3 = drop(list_2);
        assert_eq!(Some(10), head(list_3).copied());

        let list_4 = drop(list_3);
        assert_eq!(None, head(list_4).copied());
    }
}
