use crate::{
    api::ShyftApi,
    constants,
    models::{self, parsed_transaction_details::ParsedTransactionDetails},
};

impl ShyftApi {
    /// Fetches the parsed transaction details for a given transaction signature.
    ///
    /// # Arguments
    ///
    /// * `tx_signature` - A string slice representing the transaction signature.
    ///
    /// # Returns
    ///
    /// * `Result<ParsedTransactionDetails, crate::error::Error>` - A result containing the parsed transaction details or an error.
    pub async fn get_transaction_parsed(
        &self,
        tx_signature: &str,
    ) -> Result<ParsedTransactionDetails, crate::error::Error> {
        let url = format!("{}transaction/parsed", constants::URL);
        let response = self
            .client
            .get(url)
            .query(&self.default_params)
            .query(&[("txn_signature", tx_signature)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::StatusNot200(response.text().await?));
        }

        let parsed_response = response
            .json::<models::Response<ParsedTransactionDetails>>()
            .await?
            .result;
        Ok(parsed_response)
    }
}
