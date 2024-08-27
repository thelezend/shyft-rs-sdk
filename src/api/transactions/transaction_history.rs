use crate::{
    api::ShyftApi,
    constants,
    models::{self, parsed_transaction_details::ParsedTransactionDetails},
};

impl ShyftApi {
    /// Fetches the transaction history for a given account.
    ///
    /// # Arguments
    ///
    /// * `account` - A string slice representing the account address.
    /// * `tx_num` - An optional number of transactions to fetch.
    /// * `before_tx_signature` - An optional transaction signature to fetch transactions before.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ParsedTransactionDetails>, crate::error::Error>` - A result containing a vector of parsed transaction details or an error.
    pub async fn get_transaction_history(
        &self,
        account: &str,
        tx_num: Option<u32>,
        before_tx_signature: Option<&str>,
    ) -> Result<Vec<ParsedTransactionDetails>, crate::error::Error> {
        let url = format!("{}transaction/history", constants::URL);

        let mut request = self
            .client
            .get(url)
            .query(&self.default_params)
            .query(&[("account", account)])
            .query(&[("enable_raw", "true")]);

        if let Some(tx_num) = tx_num {
            request = request.query(&[("tx_num", tx_num.to_string())]);
        }

        if let Some(before_tx_signature) = before_tx_signature {
            request = request.query(&[("before_tx_signature", before_tx_signature)]);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::StatusNot200(response.text().await?));
        }

        let parsed_response = response
            .json::<models::Response<Vec<ParsedTransactionDetails>>>()
            .await?
            .result;
        Ok(parsed_response)
    }
}
