use poise::BoxFuture;
use sqlx::SqlitePool;

pub struct Database {
  pub pool: SqlitePool,
}

impl Database {
  /// Perform a transaction and return the result
  pub async fn perform_transaction<F, T>(&self, operation: F) -> Result<T, sqlx::Error>
    where
      F: for<'c> FnOnce(
        sqlx::Transaction<'c, sqlx::Sqlite>
      ) -> BoxFuture<'c, Result<T, sqlx::Error>>
  {
    // Start a new transaction
    let transaction = self.pool.begin().await?;

    // Wrap the operation in a closure that ensures rollback on error
    let result = {
      let mut tx = Some(transaction); // Wrap in Option to avoid double borrow
      let operation_result = operation(tx.take().unwrap()).await;

      match operation_result {
        Ok(value) => {
          // Commit if successful
          if let Some(tx) = tx {
            tx.commit().await?;
          }
          Ok(value)
        }
        Err(err) => {
          // Rollback on error
          if let Some(tx) = tx {
            tx.rollback().await?;
          }
          Err(err)
        }
      }
    };

    result
  }
}
