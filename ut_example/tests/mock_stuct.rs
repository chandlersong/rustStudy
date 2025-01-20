use mockall_double::double;
mod thing {
    #[cfg(test)]
    use mockall::automock;
    pub struct Thing {}
    #[automock]
    impl Thing {
        pub fn foo(&self) -> u32 {
            1
        }
    }

    pub struct AsyncThing {}

    #[automock]
    impl AsyncThing {
        pub async fn foo(&self) -> u32 {
            1
        }
    }
}

#[double]
use thing::AsyncThing;
#[double]
use thing::Thing;
fn do_stuff(thing: &Thing) -> u32 {
    thing.foo()
}

async fn do_async_stuff(thing: &AsyncThing) -> u32 {
    thing.foo().await
}

struct Staff {
    thing: Thing,
}

impl Staff {
    // can't work
    // fn new() -> Staff {
    //     Staff {
    //         thing: Thing {}
    //     }
    // }

    fn ok(&self)-> u32{
        self.thing.foo()
    }
}


#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn test_method() {
        let mut mock = Thing::default();
        mock.expect_foo().returning(|| 88);
        let actual = do_stuff(&mock);
        assert_eq!(actual, 88)
    }

    #[tokio::test]
    async fn test_async_method() {
        let mut mock = AsyncThing::default();
        mock.expect_foo().returning(|| 88);
        let actual = do_async_stuff(&mock).await;
        assert_eq!(actual, 88)
    }


    #[test]
    fn test_struct(){
        let mut mock = Thing::default();
        mock.expect_foo().returning(|| 88);
        let staff = Staff{
            thing:mock
        };
        let actual = staff.ok();
        assert_eq!(actual, 88)
    }
}
