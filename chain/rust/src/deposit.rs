use cml_core::ArithmeticError;

use crate::{
    certs::Certificate, governance::ProposalProcedure, transaction::TransactionBody, Coin, Value,
    Withdrawals,
};

pub fn internal_get_implicit_input(
    withdrawals: Option<&Withdrawals>,
    certs: Option<&[Certificate]>,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin,  // protocol parameter
) -> Result<Value, ArithmeticError> {
    let withdrawal_sum = match withdrawals {
        None => 0,
        Some(w) => w
            .values()
            .try_fold(0u64, |acc, withdrawal_amt| acc.checked_add(*withdrawal_amt))
            .ok_or(ArithmeticError::IntegerOverflow)?,
    };
    let certificate_refund = match certs {
        None => 0,
        Some(certs) => certs
            .iter()
            .try_fold(0u64, |acc, cert| match cert {
                Certificate::PoolRetirement(_cert) => acc.checked_add(pool_deposit),
                Certificate::StakeDeregistration(_cert) => acc.checked_add(key_deposit),
                Certificate::UnregCert(cert) => acc.checked_add(cert.deposit),
                Certificate::UnregDrepCert(cert) => acc.checked_add(cert.deposit),
                // TODO: is this the case?
                Certificate::ResignCommitteeColdCert(_cert) => acc.checked_add(key_deposit),
                _ => Some(acc),
            })
            .ok_or(ArithmeticError::IntegerOverflow)?,
    };

    withdrawal_sum
        .checked_add(certificate_refund)
        .ok_or(ArithmeticError::IntegerOverflow)
        .map(Value::from)
}

pub fn internal_get_deposit(
    certs: Option<&[Certificate]>,
    proposals: Option<&[ProposalProcedure]>,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin,  // protocol parameter
) -> Result<Coin, ArithmeticError> {
    let certificate_refund = match certs {
        None => 0,
        Some(certs) => certs
            .iter()
            .try_fold(0u64, |acc, cert| match cert {
                Certificate::PoolRegistration(_cert) => acc.checked_add(pool_deposit),
                Certificate::StakeRegistration(_cert) => acc.checked_add(key_deposit),
                Certificate::RegCert(cert) => acc.checked_add(cert.deposit),
                Certificate::StakeRegDelegCert(cert) => acc.checked_add(cert.deposit),
                Certificate::RegDrepCert(cert) => acc.checked_add(cert.deposit),
                Certificate::VoteRegDelegCert(cert) => acc.checked_add(cert.deposit),
                Certificate::StakeVoteRegDelegCert(cert) => acc.checked_add(cert.deposit),
                _ => Some(acc),
            })
            .ok_or(ArithmeticError::IntegerOverflow)?,
    };
    let proposal_refund = match proposals {
        None => 0,
        Some(proposals) => proposals
            .iter()
            .try_fold(0u64, |acc, proposal| acc.checked_add(proposal.deposit))
            .ok_or(ArithmeticError::IntegerOverflow)?,
    };
    certificate_refund
        .checked_add(proposal_refund)
        .ok_or(ArithmeticError::IntegerOverflow)
}

pub fn get_implicit_input(
    txbody: &TransactionBody,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin,  // protocol parameter
) -> Result<Value, ArithmeticError> {
    internal_get_implicit_input(
        txbody.withdrawals.as_ref(),
        txbody.certs.as_ref().map(|certs| certs.as_ref()),
        pool_deposit,
        key_deposit,
    )
}

pub fn get_deposit(
    txbody: &TransactionBody,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin,  // protocol parameter
) -> Result<Coin, ArithmeticError> {
    internal_get_deposit(
        txbody.certs.as_ref().map(|certs| certs.as_ref()),
        txbody
            .proposal_procedures
            .as_ref()
            .map(|proposals| proposals.as_ref()),
        pool_deposit,
        key_deposit,
    )
}
