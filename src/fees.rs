//! Program fees

use crate::bn::U256;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

/// Fees struct
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fees {
    /// Admin trade fee numerator
    pub admin_trade_fee_numerator: u64,
    /// Admin trade fee denominator
    pub admin_trade_fee_denominator: u64,
    /// Admin withdraw fee numerator
    pub admin_withdraw_fee_numerator: u64,
    /// Admin withdraw fee denominator
    pub admin_withdraw_fee_denominator: u64,
    /// Trade fee numerator
    pub trade_fee_numerator: u64,
    /// Trade fee denominator
    pub trade_fee_denominator: u64,
    /// Withdraw fee numerator
    pub withdraw_fee_numerator: u64,
    /// Withdraw fee denominator
    pub withdraw_fee_denominator: u64,
    /// Reflection fee numerator
    pub reflection_fee_numerator: u64,
    /// Reflection fee denominator
    pub reflection_fee_denominator: u64,
    /// Buyback fee numerator
    pub buyback_fee_numerator: u64,
    /// Buyback fee denominator
    pub buyback_fee_denominator: u64,
    /// Marketing fee numerator
    pub marketing_fee_numerator: u64,
    /// Marketing fee denominator
    pub marketing_fee_denominator: u64,
    /// Developer fee numerator
    pub developer_fee_numerator: u64,
    /// Developer fee denominator
    pub developer_fee_denominator: u64,
}

impl Fees {
    /// Apply admin trade fee
    pub fn admin_trade_fee(&self, fee_amount: U256) -> Option<U256> {
        fee_amount
            .checked_mul(self.admin_trade_fee_numerator.into())?
            .checked_div(self.admin_trade_fee_denominator.into())
    }

    /// Apply admin withdraw fee
    pub fn admin_withdraw_fee(&self, fee_amount: U256) -> Option<U256> {
        fee_amount
            .checked_mul(self.admin_withdraw_fee_numerator.into())?
            .checked_div(self.admin_withdraw_fee_denominator.into())
    }

    /// Compute trade fee from amount
    pub fn trade_fee(&self, trade_amount: U256) -> Option<U256> {
        trade_amount
            .checked_mul(self.trade_fee_numerator.into())?
            .checked_div(self.trade_fee_denominator.into())
    }

    /// Compute withdraw fee from amount
    pub fn withdraw_fee(&self, withdraw_amount: U256) -> Option<U256> {
        withdraw_amount
            .checked_mul(self.withdraw_fee_numerator.into())?
            .checked_div(self.withdraw_fee_denominator.into())
    }

    /// Compute reflection fee from amount
    pub fn reflection_fee(&self, reflection_amount: U256) -> Option<U256> {
        reflection_amount
            .checked_mul(self.reflection_fee_numerator.into())?
            .checked_div(self.reflection_fee_denominator.into())
    }

    /// Compute buyback fee from amount
    pub fn buyback_fee(&self, buyback_amount: U256) -> Option<U256> {
        buyback_amount
            .checked_mul(self.buyback_fee_numerator.into())?
            .checked_div(self.buyback_fee_denominator.into())
    }

    /// Compute marketing fee from amount
    pub fn marketing_fee(&self, marketing_amount: U256) -> Option<U256> {
        marketing_amount
            .checked_mul(self.marketing_fee_numerator.into())?
            .checked_div(self.marketing_fee_denominator.into())
    }

    /// Compute developer fee from amount
    pub fn developer_fee(&self, developer_amount: U256) -> Option<U256> {
        developer_amount
            .checked_mul(self.developer_fee_numerator.into())?
            .checked_div(self.developer_fee_denominator.into())
    }

    /// Compute normalized fee for symmetric/asymmetric deposits/withdraws
    pub fn normalized_trade_fee(&self, n_coins: u64, amount: U256) -> Option<U256> {
        // adjusted_fee_numerator: uint256 = self.fee * N_COINS / (4 * (N_COINS - 1))
        let adjusted_trade_fee_numerator = self
            .trade_fee_numerator
            .checked_mul(n_coins)?
            .checked_div((n_coins.checked_sub(1)?).checked_mul(4)?)?; // XXX: Why divide by 4?

        amount
            .checked_mul(adjusted_trade_fee_numerator.into())?
            .checked_div(self.trade_fee_denominator.into())
    }
}

impl Sealed for Fees {}
impl Pack for Fees {
    const LEN: usize = 128;
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, 128];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            admin_trade_fee_numerator,
            admin_trade_fee_denominator,
            admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            withdraw_fee_numerator,
            withdraw_fee_denominator,
            reflection_fee_numerator,
            reflection_fee_denominator,
            buyback_fee_numerator,
            buyback_fee_denominator,
            marketing_fee_numerator,
            marketing_fee_denominator,
            developer_fee_numerator,
            developer_fee_denominator,

        ) = array_refs![input, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8];
        Ok(Self {
            admin_trade_fee_numerator: u64::from_le_bytes(*admin_trade_fee_numerator),
            admin_trade_fee_denominator: u64::from_le_bytes(*admin_trade_fee_denominator),
            admin_withdraw_fee_numerator: u64::from_le_bytes(*admin_withdraw_fee_numerator),
            admin_withdraw_fee_denominator: u64::from_le_bytes(*admin_withdraw_fee_denominator),
            trade_fee_numerator: u64::from_le_bytes(*trade_fee_numerator),
            trade_fee_denominator: u64::from_le_bytes(*trade_fee_denominator),
            withdraw_fee_numerator: u64::from_le_bytes(*withdraw_fee_numerator),
            withdraw_fee_denominator: u64::from_le_bytes(*withdraw_fee_denominator),
            reflection_fee_numerator: u64::from_le_bytes(*reflection_fee_numerator),
            reflection_fee_denominator: u64::from_le_bytes(*reflection_fee_denominator),
            buyback_fee_numerator: u64::from_le_bytes(*buyback_fee_numerator),
            buyback_fee_denominator: u64::from_le_bytes(*buyback_fee_denominator),
            marketing_fee_numerator: u64::from_le_bytes(*marketing_fee_numerator),
            marketing_fee_denominator: u64::from_le_bytes(*marketing_fee_denominator),
            developer_fee_numerator: u64::from_le_bytes(*developer_fee_numerator),
            developer_fee_denominator: u64::from_le_bytes(*developer_fee_denominator),
        })
    }

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 128];
        let (
            admin_trade_fee_numerator,
            admin_trade_fee_denominator,
            admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            withdraw_fee_numerator,
            withdraw_fee_denominator,
            reflection_fee_numerator,
            reflection_fee_denominator,
            buyback_fee_numerator,
            buyback_fee_denominator,
            marketing_fee_numerator,
            marketing_fee_denominator,
            developer_fee_numerator,
            developer_fee_denominator,
        ) = mut_array_refs![output, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8];
        *admin_trade_fee_numerator = self.admin_trade_fee_numerator.to_le_bytes();
        *admin_trade_fee_denominator = self.admin_trade_fee_denominator.to_le_bytes();
        *admin_withdraw_fee_numerator = self.admin_withdraw_fee_numerator.to_le_bytes();
        *admin_withdraw_fee_denominator = self.admin_withdraw_fee_denominator.to_le_bytes();
        *trade_fee_numerator = self.trade_fee_numerator.to_le_bytes();
        *trade_fee_denominator = self.trade_fee_denominator.to_le_bytes();
        *withdraw_fee_numerator = self.withdraw_fee_numerator.to_le_bytes();
        *withdraw_fee_denominator = self.withdraw_fee_denominator.to_le_bytes();
        *reflection_fee_numerator = self.reflection_fee_numerator.to_le_bytes();
        *reflection_fee_denominator = self.reflection_fee_denominator.to_le_bytes();
        *buyback_fee_numerator = self.buyback_fee_numerator.to_le_bytes();
        *buyback_fee_denominator = self.buyback_fee_denominator.to_le_bytes();
        *marketing_fee_numerator = self.marketing_fee_numerator.to_le_bytes();
        *marketing_fee_denominator = self.marketing_fee_denominator.to_le_bytes();
        *developer_fee_numerator = self.developer_fee_numerator.to_le_bytes();
        *developer_fee_denominator = self.developer_fee_denominator.to_le_bytes();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_fees() {
        let admin_trade_fee_numerator = 1;
        let admin_trade_fee_denominator = 2;
        let admin_withdraw_fee_numerator = 3;
        let admin_withdraw_fee_denominator = 4;
        let trade_fee_numerator = 5;
        let trade_fee_denominator = 6;
        let withdraw_fee_numerator = 7;
        let withdraw_fee_denominator = 8;
        let reflection_fee_numerator = 9;
        let reflection_fee_denominator = 10;
        let buyback_fee_numerator = 11;
        let buyback_fee_denominator = 12;
        let marketing_fee_numerator = 13;
        let marketing_fee_denominator = 14;
        let developer_fee_numerator = 15;
        let developer_fee_denominator = 16;
        let fees = Fees {
            admin_trade_fee_numerator,
            admin_trade_fee_denominator,
            admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            withdraw_fee_numerator,
            withdraw_fee_denominator,
            reflection_fee_numerator,
            reflection_fee_denominator,
            buyback_fee_numerator,
            buyback_fee_denominator,
            marketing_fee_numerator,
            marketing_fee_denominator,
            developer_fee_numerator,
            developer_fee_denominator,
        };

        let mut packed = [0u8; Fees::LEN];
        Pack::pack_into_slice(&fees, &mut packed[..]);
        let unpacked = Fees::unpack_from_slice(&packed).unwrap();
        assert_eq!(fees, unpacked);

        let mut packed = vec![];
        packed.extend_from_slice(&admin_trade_fee_numerator.to_le_bytes());
        packed.extend_from_slice(&admin_trade_fee_denominator.to_le_bytes());
        packed.extend_from_slice(&admin_withdraw_fee_numerator.to_le_bytes());
        packed.extend_from_slice(&admin_withdraw_fee_denominator.to_le_bytes());
        packed.extend_from_slice(&trade_fee_numerator.to_le_bytes());
        packed.extend_from_slice(&trade_fee_denominator.to_le_bytes());
        packed.extend_from_slice(&withdraw_fee_numerator.to_le_bytes());
        packed.extend_from_slice(&withdraw_fee_denominator.to_le_bytes());
        packed.extend_from_slice(&reflection_fee_numerator.to_le_bytes());
        packed.extend_from_slice(&reflection_fee_denominator.to_le_bytes());
        packed.extend_from_slice(&buyback_fee_numerator.to_le_bytes());
        packed.extend_from_slice(&buyback_fee_denominator.to_le_bytes());
        packed.extend_from_slice(&marketing_fee_numerator.to_le_bytes());
        packed.extend_from_slice(&marketing_fee_denominator.to_le_bytes());
        packed.extend_from_slice(&developer_fee_numerator.to_le_bytes());
        packed.extend_from_slice(&developer_fee_denominator.to_le_bytes());
        let unpacked = Fees::unpack_from_slice(&packed).unwrap();
        assert_eq!(fees, unpacked);
    }

    #[test]
    fn fee_results() {
        let admin_trade_fee_numerator = 1;
        let admin_trade_fee_denominator = 2;
        let admin_withdraw_fee_numerator = 3;
        let admin_withdraw_fee_denominator = 4;
        let trade_fee_numerator = 5;
        let trade_fee_denominator = 6;
        let withdraw_fee_numerator = 7;
        let withdraw_fee_denominator = 8;
        let reflection_fee_numerator = 9;
        let reflection_fee_denominator = 10;
        let buyback_fee_numerator = 11;
        let buyback_fee_denominator = 12;
        let marketing_fee_numerator = 13;
        let marketing_fee_denominator = 14;
        let developer_fee_numerator = 15;
        let developer_fee_denominator = 16;
        let fees = Fees {
            admin_trade_fee_numerator,
            admin_trade_fee_denominator,
            admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            withdraw_fee_numerator,
            withdraw_fee_denominator,
            reflection_fee_numerator,
            reflection_fee_denominator,
            buyback_fee_numerator,
            buyback_fee_denominator,
            marketing_fee_numerator,
            marketing_fee_denominator,
            developer_fee_numerator,
            developer_fee_denominator,
        };

        let trade_amount = 1_000_000_000;
        let expected_trade_fee = trade_amount * trade_fee_numerator / trade_fee_denominator;
        let trade_fee = fees.trade_fee(trade_amount.into()).unwrap();
        assert_eq!(trade_fee, expected_trade_fee.into());
        let expected_admin_trade_fee =
            expected_trade_fee * admin_trade_fee_numerator / admin_trade_fee_denominator;
        assert_eq!(
            fees.admin_trade_fee(trade_fee).unwrap(),
            expected_admin_trade_fee.into()
        );

        let withdraw_amount = 100_000_000_000;
        let expected_withdraw_fee =
            withdraw_amount * withdraw_fee_numerator / withdraw_fee_denominator;
        let withdraw_fee = fees.withdraw_fee(withdraw_amount.into()).unwrap();
        assert_eq!(withdraw_fee, expected_withdraw_fee.into());
        let expected_admin_withdraw_fee =
            expected_withdraw_fee * admin_withdraw_fee_numerator / admin_withdraw_fee_denominator;
        assert_eq!(
            fees.admin_withdraw_fee(expected_withdraw_fee.into())
                .unwrap(),
            expected_admin_withdraw_fee.into()
        );

        let n_coins = 2;
        let adjusted_trade_fee_numerator = trade_fee_numerator * n_coins / (4 * (n_coins - 1));
        let expected_normalized_fee =
            U256::from(trade_amount * adjusted_trade_fee_numerator / trade_fee_denominator);
        assert_eq!(
            fees.normalized_trade_fee(n_coins, trade_amount.into())
                .unwrap(),
            expected_normalized_fee.into()
        );
    }
}
