use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock};

use std::io::Error;

// Calls to macros in the good order
#[cfg_attr(test, automock)]
#[async_trait]
pub trait ExampleTrait {
    async fn function1(&self, value: usize) -> usize;
    async fn function2(&self, value: usize) -> Result<usize, Error>;
}

#[cfg(test)]
mod tests {
    use crate::{ExampleTrait, MockExampleTrait};


    /**
    https://medium.com/vortechsa/mocking-in-async-rust-248b012c5e99
    基于这篇文章的写法
    */
    #[tokio::test]
    async fn test() {
        let mut mock = MockExampleTrait::new();

        mock.expect_function1().times(1).returning(|_| 10);

        mock.expect_function2().times(1).returning(|_| Ok(11));

        let result1 = mock.function1(110).await;
        assert_eq!(result1, 10);

        let result2 = mock.function2(111).await.expect("Got an error");

        assert_eq!(result2, 11);
    }
}
