extern crate tdo_core;


#[cfg(test)]
mod interact {
    use tdo_core::{todo, list};

    #[test]
    fn clean_list() {
        let mut tdo = list::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        let mut tdo2 = list::Tdo::new();
        tdo2.add_list(list::TodoList::new("test")).unwrap();
        tdo2.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        tdo.lists[1].done_id(0).unwrap();
        tdo.lists[1].clean();
        assert_eq!(tdo.lists[1].list.len(), tdo2.lists[1].list.len());
    }

    #[test]
    #[should_panic]
    fn add_to_not_existing_list() {
        let mut tdo = list::Tdo::new();
        let res = tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        assert_eq!(res, ())
    }

    #[test]
    fn set_done_list() {
        let mut tdo = list::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        tdo.lists[1].done_id(0).unwrap();
        assert!(tdo.lists[1].list[0].done);
    }

    #[test]
    fn clean_tdo() {
        let mut tdo = list::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        let mut tdo2 = list::Tdo::new();
        tdo2.add_list(list::TodoList::new("test")).unwrap();
        tdo2.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        tdo.done_id(0);
        tdo.clean_lists();
        assert_eq!(tdo.lists[1].list.len(), tdo2.lists[1].list.len());
    }

    #[test]
    fn set_done_tdo() {
        let mut tdo = list::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        tdo.done_id(0);
        assert!(tdo.lists[1].list[0].done);
    }

}
